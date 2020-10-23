/*
 * A step which will invoke steps inside a directory
 */

use serde::Deserialize;
use serde_yaml::Value;
use std::fs::File;

#[derive(Clone, Debug, Deserialize)]
struct Invocation {
    parameters: Parameters,
}

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    directory: String,
    block: Vec<Step>,
}

#[derive(Clone, Debug, Deserialize)]
struct Step {
    symbol: String,
    parameters: Value,
}
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("The dir step can only accept a single argument: the invocation file path");
    }

    let file = File::open(&args[1])?;

    match serde_yaml::from_reader::<File, Invocation>(file) {
        Err(e) => {
            panic!("Failed to parse invocation file: {:#?}", e);
        }
        Ok(invoke) => {
            // do things
            Ok(())
        }
    }
}
