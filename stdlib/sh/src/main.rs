/*
 * A very simple step which just invokes a shell script with some flags
 */

use os_pipe::pipe;
use serde::Deserialize;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
use tempfile::NamedTempFile;

use otto_agent::step::*;

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

    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-xe");
    cmd.arg(file.path());

    let (reader, writer) = pipe().unwrap();
    let writer_clone = writer.try_clone().unwrap();
    cmd.stdout(writer);
    cmd.stderr(writer_clone);

    let mut handle = cmd.spawn()?;
    drop(cmd);

    let bufr = BufReader::new(reader);
    for line in bufr.lines() {
        if let Ok(buffer) = line {
            println!("{}", buffer);
        }
    }

    let status = handle.wait()?;

    std::process::exit(
        status
            .code()
            .expect("Could not get exit code from subprocess"),
    );
}
