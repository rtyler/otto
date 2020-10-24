use log::*;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{stderr, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;
use uuid::Uuid;

/**
 * A Pipeline contains the total configuration and steps for a single pipeline run
 */
#[derive(Clone, Debug, Deserialize)]
pub struct Pipeline {
    #[serde(default = "generate_uuid")]
    pub uuid: Uuid,
    pub contexts: Vec<Context>,
    pub steps: Vec<Step>,
}

/**
 * A context is some bucket of variables and configuration within a pipeline
 * this will most frequently be a "stage" in the conventional sense
 */
#[derive(Clone, Debug, Deserialize)]
pub struct Context {
    #[serde(default = "generate_uuid")]
    pub uuid: Uuid,
    pub name: String,
    pub environment: Option<HashMap<String, String>>,
}

/**
 * A step is the smallest unit of execution for the pipeline
 */
#[derive(Clone, Debug, Deserialize)]
pub struct Step {
    #[serde(default = "generate_uuid")]
    pub uuid: Uuid,
    /// The uuid of the context to which this step is associated
    pub context: Uuid,
    pub symbol: String,
    pub parameters: Value,
}

/**
 * Log is a data structure which captures the necessary metadata for logging a single line
 */
#[derive(Clone, Debug, Serialize)]
pub enum Log {
    StepStart {
        symbol: String,
        uuid: Uuid,
    },
    StepOutput {
        symbol: String,
        uuid: Uuid,
        buffer: String,
        stream: LogStream,
    },
    StepEnd {
        symbol: String,
        uuid: Uuid,
    },
}

#[derive(Clone, Debug, Serialize)]
pub enum LogStream {
    Stdout,
    Stderr,
}

/**
 * Generate a UUID v4 for use in structs, etc
 */
fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

/**
 * The run method is the "core" of the agent which will run a series of steps
 * passed in.
 *
 * Currently it is very simple and primitive
 */
pub fn run(steps_dir: &str, steps: &Vec<Step>) -> std::io::Result<()> {
    let dir = Path::new(steps_dir);

    if !dir.is_dir() {
        panic!("STEPS_DIR must be a directory! {:?}", dir);
    }

    let mut manifests: HashMap<String, osp::Manifest> = HashMap::new();
    let mut m_paths: HashMap<String, PathBuf> = HashMap::new();

    for step in steps.iter() {
        let manifest_file = dir.join(&step.symbol).join("manifest.yml");

        if manifest_file.is_file() {
            let file = File::open(manifest_file)?;
            // TODO: This is dumb and inefficient
            m_paths.insert(step.symbol.clone(), dir.join(&step.symbol).to_path_buf());
            manifests.insert(
                step.symbol.clone(),
                serde_yaml::from_reader::<File, osp::Manifest>(file)
                    .expect("Failed to parse manifest"),
            );
        } else {
            warn!(
                "{}/manifest.yml does not exist, step cannot execute",
                step.symbol
            );
        }
    }

    // Now that things are valid and collected, let's executed
    for step in steps.iter() {
        if let Some(runner) = manifests.get(&step.symbol) {
            let m_path = m_paths
                .get(&step.symbol)
                .expect("Failed to grab the step library path");
            let entrypoint = m_path.join(&runner.entrypoint.path);

            let mut file = NamedTempFile::new()?;
            let mut step_args = HashMap::new();
            step_args.insert("parameters", &step.parameters);

            serde_yaml::to_writer(&mut file, &step_args)
                .expect("Failed to write temporary file for script");

            use os_pipe::pipe;
            use std::io::{BufRead, BufReader};
            let mut cmd = Command::new(entrypoint);
            cmd.arg(file.path());
            let (mut reader, writer) = pipe().unwrap();
            let writer_clone = writer.try_clone().unwrap();
            cmd.stdout(writer);
            cmd.stderr(writer_clone);

            let log = Log::StepStart {
                symbol: step.symbol.clone(),
                uuid: step.uuid,
            };
            println!("{:?}", log);

            let mut handle = cmd.spawn()?;
            drop(cmd);

            let bufr = BufReader::new(reader);
            for line in bufr.lines() {
                if let Ok(buffer) = line {
                    if "dir" == step.symbol {
                        println!("{}", buffer);
                    } else {
                        let log = Log::StepOutput {
                            // TODO: Remove this allocation
                            symbol: step.symbol.clone(),
                            uuid: step.uuid,
                            stream: LogStream::Stdout,
                            buffer,
                        };
                        // TODO: send this to a log service
                        println!("{:?}", log);
                    }
                }
            }

            handle.wait()?;

            let log = Log::StepEnd {
                symbol: step.symbol.clone(),
                uuid: step.uuid,
            };
            println!("{:?}", log);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
