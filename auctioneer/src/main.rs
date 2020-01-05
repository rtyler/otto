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
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::*;
use url::Url;

use otto_eventbus::*;

struct BusClient {}

impl Actor for BusClient {
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Starting client");
    }
}

/**
 * Simple websocket connection function, just to allow for easy reconnections
 */
fn ws_connect() -> Result<(WebSocket<AutoStream>, Response)> {
    return connect(Url::parse("ws://localhost:8000/ws/").unwrap());
}

#[actix_rt::main]
async fn main() {
    pretty_env_logger::init();

    let (mut socket, _response) = ws_connect().unwrap();

    info!("Connected to the server");

    /*
     * In this loop we should read the message, and then dispatch it to a handler actor immediately
     * to do "the work"
     */
    loop {
        let msg = socket.read_message();
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to read a message off the WebSocket: {:?}", e);
                // must reconnect
                thread::sleep(Duration::from_secs(3));

                /*
                 * Ignore connection errors, they're not important since we
                 * are in the retry loop anyways.
                 */
                if let Ok(recon) = ws_connect() {
                    info!("Server reconnected");
                    socket = recon.0;
                }
                continue;
            }
        };
        info!("Received: {}", msg);
    }
}
