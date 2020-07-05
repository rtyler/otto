extern crate otto_eventbus;

use log::*;
use otto_eventbus::message::*;
use serde_json;
use tungstenite::Message;
use uuid::Uuid;

mod common;
use common::*;

/**
 * This test will ensure that the server properly responds to N+1 registration
 * requests
 */
#[async_std::test]
async fn register_test() -> std::io::Result<()> {
    pretty_env_logger::init();
    let _ = bootstrap_server();

    if let Ok((mut socket, _response)) = ws_connect() {
        info!("Connected to the server");
        let uuid =  Uuid::new_v4();
        let _token = register_client(uuid, &mut socket);

        let register = Register {
            uuid,
            token: None,
        };

        let envelope = meows::Envelope {
            ttype: "register".to_string(),
            value: serde_json::to_value(&register).expect("Failed to convert to value"),
        };
        let buffer = serde_json::to_string(&envelope).expect("Failed to serialize the register");

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
