/*
 * The error step is really really really simple
 */

use otto_agent::step::*;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    message: String,
}

fn main() {
    let args = std::env::args().collect();
    let invoke: Invocation<Parameters> = invocation_from_args(&args).unwrap();

    println!("{}", invoke.parameters.message);
    std::process::exit(1);
}
