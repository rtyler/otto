/*
 * A step which will invoke steps inside a directory
 */

use ottoagent::step::*;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    directory: String,
    block: Vec<ottoagent::Step>,
}

fn main() -> std::io::Result<()> {
    let steps_dir = std::env::var("STEPS_DIR").expect("STEPS_DIR must be defined");
    let args = std::env::args().collect();
    let invoke: Invocation<Parameters> = invocation_from_args(&args).unwrap();

    std::env::set_current_dir(&invoke.parameters.directory)
        .expect("Failed to set current directory, perhaps it doesn't exist");

    // Construct a somewhat fabricated pipeline configuration
    let pipeline = ottoagent::Pipeline {
        uuid: invoke.configuration.pipeline,
        contexts: vec![],
        steps: invoke.parameters.block,
    };

    let status = ottoagent::run(&steps_dir, &pipeline, None).unwrap();
    // Pass our block-scoped status back up to the caller
    std::process::exit(status as i32);
}
