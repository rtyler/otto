/*
 * This file contains the entrypoint for the agent binary.
 *
 * Most of the logic _should_ be contained within lib.rs and the surrounding modules
 */
use async_std::sync::channel;
use std::fs::File;

use otto_agent::*;

/**
 * The maximum number of pending controll messages allowed
 *
 * If the number of pending messages exceeds this number, the requests to the
 * control socket will block until the pending messages are cleared out
 */
const MAX_CONTROL_MSGS: usize = 64;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    let steps_dir = std::env::var("STEPS_DIR").expect("STEPS_DIR must be defined");

    if args.len() != 2 {
        panic!("The sh step can only accept a single argument: the parameters file path");
    }

    let file = File::open(&args[1])?;
    let (sender, receiver) = channel(MAX_CONTROL_MSGS);

    match serde_yaml::from_reader::<File, otto_models::Pipeline>(file) {
        Err(e) => {
            panic!("Failed to parse parameters file: {:#?}", e);
        }
        Ok(invoke) => {
            async_std::task::spawn(async {
                // TODO better error handling and behavior
                control::run(sender).await.expect("Failed to bind control?");
            });

            run(&steps_dir, &invoke, Some(receiver)).expect("Failed to run pipeline");
        }
    };
    Ok(())
}
