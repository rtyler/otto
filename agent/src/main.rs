use std::fs::File;

use ottoagent::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let steps_dir = std::env::var("STEPS_DIR").expect("STEPS_DIR must be defined");

    if args.len() != 2 {
        panic!("The sh step can only accept a single argument: the parameters file path");
    }

    let file = File::open(&args[1])?;

    match serde_yaml::from_reader::<File, Pipeline>(file) {
        Err(e) => {
            panic!("Failed to parse parameters file: {:#?}", e);
        }
        Ok(invoke) => {
            run(&steps_dir, &invoke.steps);
        }
    };
    Ok(())
}
