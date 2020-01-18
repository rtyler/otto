fn main() {}
/*
 * The CLI is meant to be used for manual testing and verification of the eventbus only.
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::*;
use url::Url;

fn ws_connect() -> Result<(WebSocket<AutoStream>, Response)> {
    return connect(Url::parse("ws://localhost:8000/ws/").unwrap());
}

fn main() {
    let (mut socket, _response) = ws_connect().unwrap();
    println!("Connected to the server");

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let history = ".otto-ebc-history";

    if rl.load_history(history).is_err() {
        println!("No previous history");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.len() == 0 {
                    let response = socket.read_message();
                    match response {
                        Ok(msg) => {
                            println!("{}", msg);
                        }
                        Err(e) => {
                            println!("Failed to read: {}", e);
                        }
                    }
                } else {
                    rl.add_history_entry(line.as_str());
                    if line == "quit" {
                        return;
                    }
                    match socket.write_message(Message::Text(line)) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Failed to write: {}", e);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // ctrl-C
                break;
            }
            Err(ReadlineError::Eof) => {
                // ctrl-D
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(history).unwrap();
}
*/
