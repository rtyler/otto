extern crate eventbus_inmemory;

use futures::channel::mpsc::Sender;
use log::*;
use otto_eventbus::message::*;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::*;
use url::Url;
use uuid::Uuid;

/** Simple test function for connecting to a websocket server
 */
pub fn ws_connect() -> Result<(WebSocket<AutoStream>, Response)> {
    return connect(Url::parse("ws://127.0.0.1:8105/").unwrap());
}

/**
 * This function will spawn the eventbus server in a background thread
 */
pub fn bootstrap_server() -> Sender<meows::Control> {
    // This looks like it can only be called once per process invocation
    //pretty_env_logger::init();
    let server = eventbus_inmemory::create_server();
    let controller = server.get_control_channel();
    let addr = "127.0.0.1:8105".to_string();

    /*
     * Run the in-memory server inside of a thread to allow the tests to operate
     * on the main thread
     */
    std::thread::spawn(move || {
        smol::run(
            // TODO: Make this bind to port 0 to get a properly random server port
            eventbus_inmemory::run_server(server, addr),
        )
    });

    /*
     * Give the server a second to come up and bind to the socket
     */
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Give the caller the server controller so that they may terminate
    controller
}

/**
 * Register the client and send the client's new auth token back
 *
 * This function WILL make assertions on the responses
 *
 * Should return the auth token, otherwise None in case of any failures
 */
pub fn register_client(uuid: Uuid, socket: &mut WebSocket<AutoStream>) -> Option<Uuid> {
    let register = Register { uuid, token: None };
    let buffer = wrap_in_envelope("register".to_string(), &register);

    socket
        .write_message(Message::text(&buffer))
        .expect("Failed to send message to test server");

    if let Ok(m) = socket.read_message() {
        info!("Read from server: {:?}", m);
        assert!(m.is_text());

        /*
         * This should be a "registered" response message and the test should
         * verify that
         */
        let text = m
            .into_text()
            .expect("Failed to convert message payload to text");
        let registered: Registered =
            serde_json::from_str(&text).expect("Failed to deserialize the response");

        assert!(!registered.token.is_nil());
        assert!(register.uuid != registered.token);
        return Some(registered.token);
    }
    None
}

/**
 * This function will wrap the given message in a meows envelope and give you
 * the string you need to write to a socket back
 */
pub fn wrap_in_envelope<T: serde::ser::Serialize>(name: String, message: &T) -> String {
    let envelope = meows::Envelope {
        ttype: name,
        value: serde_json::to_value(&message).expect("Failed to convert to value"),
    };
    serde_json::to_string(&envelope).expect("Failed to serialize the register")
}
