/**!
 * The auctioneer main model
 */
extern crate actix;
extern crate pretty_env_logger;
extern crate tungstenite;
use std::time::Duration;
use std::{io, thread};

use actix::*;
use log::*;
use tungstenite::{connect, Message};
use url::Url;

use otto_eventbus::*;

struct BusClient {}

impl Actor for BusClient {
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Starting client");
    }
}

#[actix_rt::main]
async fn main() {
    pretty_env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:8000/ws/").unwrap()).expect("Can't connect");

    println!("Connected to the server");

    //socket
    //    .write_message(Message::Text("Hello WebSocket".into()))
    //    .unwrap();

    /*
     * In this loop we should read the message, and then dispatch it to a handler actor immediately
     * to do "the work"
     */
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
