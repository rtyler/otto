/*
 * This file contains the entrypoint for the agent binary.
 *
 * Most of the logic _should_ be contained within lib.rs and the surrounding modules
 */
use async_std::sync::channel;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use uuid::Uuid;

use otto_agent::*;

/**
 * The maximum number of pending controll messages allowed
 *
 * If the number of pending messages exceeds this number, the requests to the
 * control socket will block until the pending messages are cleared out
 */
const MAX_CONTROL_MSGS: usize = 64;

/**
 * The format of the invocation file for the agent
 */
#[derive(Clone, Debug, Deserialize)]
struct Invocation {
    pipeline: Uuid,
    steps: Vec<otto_models::Step>,
}

/**
 * Ensure the directory exists by making it or panicking
 */
fn mkdir_if_not_exists(path: &Path) -> std::io::Result<()> {
    use std::io::{Error, ErrorKind};

    if path.exists() {
        if path.is_dir() {
            return Ok(())
        }
        return Err(Error::new(ErrorKind::AlreadyExists, format!("{:?} exists and is not a directory", path)));
    }
    else {
        std::fs::create_dir(path)?;
    }
    Ok(())
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    let steps_dir = std::env::var("STEPS_DIR").expect("STEPS_DIR must be defined");

    if args.len() != 2 {
        panic!("The agent can only accept a single argument: the invocation file path");
    }

    let file = File::open(&args[1])?;

    let work_dir = Path::new("agent-work");
    let cache_dir = work_dir.join("caches");
    mkdir_if_not_exists(&work_dir)?;
    mkdir_if_not_exists(&cache_dir)?;

    std::env::set_current_dir(work_dir)?;

    let (sender, receiver) = channel(MAX_CONTROL_MSGS);

    match serde_json::from_reader::<File, Invocation>(file) {
        Err(e) => {
            panic!("Failed to parse parameters file: {:#?}", e);
        }
        Ok(invoke) => {
            async_std::task::spawn(async {
                // TODO better error handling and behavior
                control::run(sender).await.expect("Failed to bind control?");
            });

            /*
             * Enter into the pipeline specific work directory
             */
            let pipeline_dir = invoke.pipeline.to_hyphenated().to_string();
            let pipeline_dir = Path::new(&pipeline_dir);
            mkdir_if_not_exists(&pipeline_dir)?;
            std::env::set_current_dir(pipeline_dir)?;

            run(&steps_dir, &invoke.steps, invoke.pipeline, Some(receiver))
                .expect("Failed to run pipeline");
        }
    };
    Ok(())
}
