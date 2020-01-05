/**
 * The CLI is meant to be used for manual testing and verification of the eventbus only.
 */

use std::io::{stdin,stdout,Write};
use tungstenite::*;
use url::Url;

fn main() {
    let (mut socket, response) = connect(Url::parse("ws://localhost:8000/ws/").unwrap()).expect("Failed to connect");
    println!("Connected to the server");

    loop {
        let mut message = String::new();
        print!("> ");
        let _ = stdout().flush();

        let _ = stdin().read_line(&mut message).unwrap();


        if let Some('\n') = message.chars().next() {
            let msg = socket.read_message().expect("Failed to read message");
            println!("Received: {}", msg);
        }
        else {
            socket.write_message(Message::Text(message));
        }
    }
}
