extern crate bastion;
extern crate num_cpus;
extern crate pretty_env_logger;
extern crate tungstenite;
extern crate url;

use bastion::prelude::*;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tungstenite::{connect, Message};
use url::Url;

use log::*;

fn main() {
    pretty_env_logger::init();
    Bastion::init();

    let supervisor = Bastion::supervisor(|sp| sp.with_strategy(SupervisionStrategy::OneForOne))
        .expect("Couldn't create the supervisor.");

    let workers = supervisor
        .children(|children: Children| {
            children
                .with_redundancy(num_cpus::get())
                .with_exec(move |ctx: BastionContext| {
                    async move {
                        info!("Starting child {:?}", ctx.current().id());
                        loop {
                            msg! { ctx.recv().await?,
                                msg: Message => {
                                    info!("Received message {} on {:?}", msg, ctx.current().id());
                                };
                                _: _ => {
                                    ()
                                };
                            }
                        }
                    }
                })
        })
        .expect("Couldn't start workers group");

    let workers = Arc::new(workers);

    supervisor
        .children(|children: Children| {
            children.with_exec(move |_ctx: BastionContext| {
                let workers = workers.clone();
                async move {
                    info!("Server is starting!");

                    let (mut socket, response) =
                        connect(Url::parse("ws://localhost:8000/ws/").unwrap())
                            .map_err(|e| {
                                error!("Failed to connect! {:?}", e);
                                sleep(Duration::from_secs(1));
                            })
                            .expect("Can't connect");

                    info!("Response HTTP code: {}", response.code);
                    info!("Response contains the following headers:");
                    for (ref header, _value) in response.headers.iter() {
                        info!("* {}", header);
                    }

                    socket
                        .write_message(Message::Text("Hello WebSocket".into()))
                        .unwrap();

                    loop {
                        for worker in workers.elems().iter() {
                            let msg = socket.read_message().expect("Error reading message");
                            info!("telling {:?}", worker.id());
                            worker.tell(msg).expect("Failed to tell worker to do work!");
                        }
                    }
                }
            })
        })
        .expect("Couldn't start a new children group.");

    Bastion::start();
    Bastion::block_until_stopped();
}
