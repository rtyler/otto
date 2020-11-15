/*
 * The git step does simple clones of git repositories
 */
use serde::Deserialize;
use otto_agent::step::*;

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    url: String,
    r#ref: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect();
    let invoke: Invocation<Parameters> = invocation_from_args(&args).unwrap();

    Ok(())
}
