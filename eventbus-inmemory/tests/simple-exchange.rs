/**
 * This integration test is intended to support just a simple exchange between
 * the eventbus and a websocket client
 */

extern crate eventbus_inmemory;

use async_std::task;
use log::*;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::*;
use url::Url;

fn ws_connect() -> Result<(WebSocket<AutoStream>, Response)> {
    return connect(Url::parse("ws://127.0.0.1:8105/").unwrap());
}

#[async_std::test]
async fn simple_connect() -> std::io::Result<()> {
    pretty_env_logger::init();
    std::thread::spawn(|| {
        println!("YOLO");
        smol::run(
            eventbus_inmemory::run_server("127.0.0.1:8105".to_string())
        )
    });

    let (mut socket, _response) = ws_connect().unwrap();
    println!("Connected to the server");

    Ok(())
}
