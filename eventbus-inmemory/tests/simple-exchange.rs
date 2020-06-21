/**
 * This integration test is intended to support just a simple exchange between
 * the eventbus and a websocket client
 */

use async_std::task;

use log::*;
use otto_eventbus::server::*;

use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::*;
use url::Url;


#[async_std::test]
async fn simple_connect() -> std::io::Result<()> {

    assert!(false);
    Ok(())
}
