use log::*;
use otto_eventbus::message::*;
use tungstenite::Message;
use uuid::Uuid;

mod common;
use common::*;

/**
 * Register and subscribe to a channel
 */
#[async_std::test]
async fn register_and_subscribe() -> std::io::Result<()> {
    pretty_env_logger::init();
    let _ = bootstrap_server();

    if let Ok((mut socket, _response)) = ws_connect() {
        info!("Connected to the server");
        let client = Uuid::new_v4();
        let auth = register_client(client, &mut socket).expect("Failed to get auth token");

        let header = ClientHeader {
            uuid: client,
            token: auth,
        };
        let channel = "/internal/echo".to_string();

        /*
         * Subscribing to the internal echo channel to ensure that we get whatever
         * messages we had sent back
         */
        let subscribe = Subscribe {
            channel: channel.clone(),
            header: header.clone(),
        };
        let buffer = wrap_in_envelope("subscribe".to_string(), &subscribe);

        socket
            .write_message(Message::text(buffer))
            .expect("Failed to send message to test server");

        let value = serde_json::from_str(r#"{"hello":"world"}"#)
            .expect("Failed to generate test value");
        let publish = Publish {
            header: header.clone(),
            channel: channel.clone(),
            value: value,
        };
        let buffer = wrap_in_envelope("publish".to_string(), &publish);
        info!("Sending publish buffer: {}", buffer);

        socket
            .write_message(Message::text(buffer))
            .expect("Failed to send message to test server");

        if let Ok(m) = socket.read_message() {
            info!("Read from server: {:?}", m);
        }

        socket
            .close(None)
            .expect("Failed to cleanly close connection");
    } else {
        error!("Failed to connect to the websocket server");
        assert!(false);
    }

    Ok(())
}
