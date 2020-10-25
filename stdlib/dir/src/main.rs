/*
 * A step which will invoke steps inside a directory
 */

use serde::Deserialize;
use std::fs::File;

use ottoagent::*;

#[derive(Clone, Debug, Deserialize)]
struct Invocation {
    parameters: Parameters,
}

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    directory: String,
    block: Vec<Step>,
}

fn main() -> std::io::Result<()> {
    let steps_dir = std::env::var("STEPS_DIR").expect("STEPS_DIR must be defined");

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
            std::env::set_current_dir(&invoke.parameters.directory)
                .expect("Failed to set current directory, perhaps it doesn't exist");
            run(&steps_dir, &invoke.parameters.block, None);
            Ok(())
        }
    }
}
