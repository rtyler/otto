extern crate bastion;
extern crate pretty_env_logger;
extern crate tungstenite;
extern crate url;

use bastion::prelude::*;
use tungstenite::{connect, Message};
use url::Url;

use otto_eventbus::Command;

fn main() {
    pretty_env_logger::init();
    // Creating the system's configuration...
    let config = Config::new().hide_backtraces();
    // ...and initializing the system with it (this is required)...
    Bastion::init_with(config);

    // Note that `Bastion::init();` would work too and initialize
    // the system with the default config.


    let supervisor = Bastion::supervisor(|sp| {
        sp.with_strategy(SupervisionStrategy::OneForOne)
    }).expect("Couldn't create the supervisor.");

    Bastion::children(|children: Children| {
        children.with_exec(move |ctx: BastionContext| {
            async move {
                println!("Server is starting!");

                let (mut socket, response) =
                    connect(Url::parse("ws://localhost:8000/ws/").unwrap()).expect("Can't connect");

                println!("Connected to the server");
                println!("Response HTTP code: {}", response.code);
                println!("Response contains the following headers:");
                for (ref header, _value) in response.headers.iter() {
                    println!("* {}", header);
                }

                socket
                    .write_message(Message::Text("Hello WebSocket".into()))
                    .unwrap();
                loop {
                    let msg = socket.read_message().expect("Error reading message");
                    println!("Received: {}", msg);
                }
                // socket.close(None);

                // Send a signal to system that computation is finished.
                Bastion::stop();

                Ok(())
            }
        })
    })
    .expect("Couldn't start a new children group.");


    // Starting the system...
    Bastion::start();
    Bastion::block_until_stopped();
}
