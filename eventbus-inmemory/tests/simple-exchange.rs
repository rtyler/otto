/**
 * This integration test is intended to support just a simple exchange between
 * the eventbus and a websocket client
 */

extern crate otto_eventbus;
extern crate eventbus_inmemory;

use log::*;
use otto_eventbus::message::*;
use serde_json;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::*;
use uuid::Uuid;
use url::Url;

/** Simple test function for connecting to a websocket server
 */
fn ws_connect() -> Result<(WebSocket<AutoStream>, Response)> {
    return connect(Url::parse("ws://127.0.0.1:8105/").unwrap());
}

#[async_std::test]
async fn simple_connect() -> std::io::Result<()> {
    pretty_env_logger::init();
    std::thread::spawn(|| {
        smol::run(
            eventbus_inmemory::run_server("127.0.0.1:8105".to_string())
        )
    });

    /*
     * Give the server a second to come up and bind to the socket
     */
    std::thread::sleep(std::time::Duration::from_secs(1));

    if let Ok((mut socket, _response)) = ws_connect() {
        info!("Connected to the server");
        let register = Register {
            uuid: Uuid::new_v4(),
            token: None,
        };

        let envelope = meows::Envelope {
            ttype: "register".to_string(),
            value: serde_json::to_value(&register).expect("Failed to convert to value"),
        };
        let buffer = serde_json::to_string(&envelope).expect("Failed to serialize the register");

        socket.write_message(Message::text(&buffer))
            .expect("Failed to send message to test server");

        if let Ok(m) = socket.read_message() {
            info!("Read from server: {:?}", m);
            assert!(m.is_text());

            /*
             * This should be a "registered" response message and the test should
             * verify that
             */
            let text = m.into_text().expect("Failed to convert message payload to text");
            let registered: Registered = serde_json::from_str(&text).expect("Failed to deserialize the response");

            assert!(!registered.token.is_nil());
            assert!(register.uuid != registered.token);
        }

        /*
         * On the second iteration, the register event should return an error
         */
        socket.write_message(Message::text(buffer))
            .expect("Failed to send message to test server");
        if let Ok(m) = socket.read_message() {
            info!("Read second response from server: {:?}", m);
            assert!(m.is_text());
            let text = m.into_text().expect("Failed to convert message payload to text");
            let error: otto_eventbus::message::Error = serde_json::from_str(&text).expect("Failed to deserialize the response");
            assert_eq!(error.code, "eventbus.already_registered");
        }

        socket.close(None)
            .expect("Failed to cleanly close connection");
    }
    else {
        error!("Failed to connect to local server");
        assert!(false);
    }

    Ok(())
}
