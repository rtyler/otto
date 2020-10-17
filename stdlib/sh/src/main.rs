/*
 * A very simple step which just invokes a shell script with some flags
 */

use serde::Deserialize;
use std::fs::File;
use std::io::{stderr, stdout, Write};
use std::process::Command;
use tempfile::NamedTempFile;

#[derive(Clone, Debug, Deserialize)]
struct Config {
    parameters: Parameters,
}

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
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("The sh step can only accept a single argument: the parameters file path");
    }

    let file = File::open(&args[1])?;

    match serde_yaml::from_reader::<File, Config>(file) {
        Err(e) => {
            panic!("Failed to parse parameters file: {:#?}", e);
        },
        Ok(config) => {
            // Create a file inside of `std::env::temp_dir()`.
            let mut file = NamedTempFile::new()?;
            writeln!(file, "{}", config.parameters.script)
                .expect("Failed to write temporary file for script");


            let output = Command::new("/bin/sh")
                            .arg(file.path())
                            .output()
                            .expect("Failed to invoke the script");

            stdout().write_all(&output.stdout).unwrap();
            stderr().write_all(&output.stderr).unwrap();

            if output.status.success() {
                return Ok(());
            }
            else {
                std::process::exit(output.status.code().expect("Failed to get status code of script"));
            }
        },
    }
}
