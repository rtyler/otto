/*
 * A very simple step which just invokes a shell script with some flags
 */

use serde::Deserialize;
use std::io::{stderr, stdout, Write};
use std::process::Command;
use tempfile::NamedTempFile;

use ottoagent::step::*;

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    script: String,
    encoding: Option<String>,
    label: Option<String>,
    #[serde(rename = "returnStatus")]
    return_status: Option<bool>,
    #[serde(rename = "returnStdout")]
    return_stdout: Option<bool>,
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect();
    let invoke: Invocation<Parameters> = invocation_from_args(&args).unwrap();

    // Create a file inside of `std::env::temp_dir()`.
    let mut file = NamedTempFile::new()?;
    writeln!(file, "{}", invoke.parameters.script)
        .expect("Failed to write temporary file for script");

    let output = Command::new("/bin/sh")
        .arg("-xe")
        .arg(file.path())
        .output()
        .expect("Failed to invoke the script");

    stdout().write_all(&output.stdout).unwrap();
    stderr().write_all(&output.stderr).unwrap();

    std::process::exit(
        output
            .status
            .code()
            .expect("Failed to get status code of script"),
    );
}
