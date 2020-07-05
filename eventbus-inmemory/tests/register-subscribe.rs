use log::*;
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
        let _auth = register_client(client, &mut socket);
        socket
            .close(None)
            .expect("Failed to cleanly close connection");
    } else {
        error!("Failed to connect to the websocket server");
        assert!(false);
    }

    Ok(())
}
