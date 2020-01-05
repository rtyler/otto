/**
 * The CLI is meant to be used for manual testing and verification of the eventbus only.
 */

extern crate rustyline;

use std::io::{stdin, stdout, Write};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use tungstenite::*;
use url::Url;

fn main() {
    let (mut socket, response) =
        connect(Url::parse("ws://localhost:8000/ws/").unwrap()).expect("Failed to connect");
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
                    let msg = socket.read_message().expect("Failed to read message");
                    println!("Received: {}", msg);
                } else {
                    socket.write_message(Message::Text(line));
                }
            },
            Err(ReadlineError::Interrupted) => {
                // ctrl-C
                break
            },
            Err(ReadlineError::Eof) => {
                // ctrl-D
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(history).unwrap();
}
