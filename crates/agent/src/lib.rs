use async_std::sync::Receiver;
use log::*;
use otto_models::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::NamedTempFile;
use uuid::Uuid;

pub mod control;
pub mod step;

/**
 * The format of the invocation file for the agent
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invocation {
    pub pipeline: Uuid,
    pub steps: Vec<otto_models::Step>,
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

#[derive(Clone, Debug)]
struct LoadedManifest {
    manifest: osp::Manifest,
    path: PathBuf,
}

fn load_manifests_for_symbols(
    steps_dir: &str,
    symbols: Vec<String>,
) -> std::io::Result<HashMap<String, LoadedManifest>> {
    use std::io::{Error, ErrorKind};

    let dir = Path::new(steps_dir);
    let dir = std::fs::canonicalize(dir)?;

    if !dir.is_dir() {
        error!("STEPS_DIR must be a directory! {:?}", dir);
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "STEPS_DIR not a directory",
        ));
    }

    let mut manifests = HashMap::new();

    for step in symbols.iter() {
        let manifest_file = dir.join(step).join("manifest.yml");

        if manifest_file.is_file() {
            let file = File::open(manifest_file)?;
            if let Ok(manifest) = serde_yaml::from_reader::<File, osp::Manifest>(file) {
                let manifest = LoadedManifest {
                    manifest,
                    path: dir.join(step).to_path_buf(),
                };
                manifests.insert(step.clone(), manifest);
            }
        } else {
            warn!("{}/manifest.yml does not exist, step cannot execute", step);
        }
    }
    Ok(manifests)
}

fn load_manifests_for(
    steps_dir: &str,
    steps: &Vec<Step>,
) -> std::io::Result<HashMap<String, LoadedManifest>> {
    load_manifests_for_symbols(steps_dir, steps.iter().map(|s| s.symbol.clone()).collect())
}

/**
 * This conveninece function will just generate the endpoint with the object store URL for the
 * given pipeline
 */
fn object_endpoint_for(uuid: &Uuid) -> step::Endpoint {
    step::Endpoint {
        url: url::Url::parse(&format!("http://localhost:7671/{}", uuid))
            .expect("Failed for prepare the object endpoint for a pipeline"),
    }
}

/**
 * Convert positional StepParameters to keywords based on the manifest for the step
 *
 * This function will clone the parameters
 */
fn positional_to_keyword(args: &StepParameters, manifest: &osp::Manifest) -> StepParameters {
    match args {
        StepParameters::Keyword(_) => args.clone(),
        StepParameters::Positional(args) => {
            let mut kwargs = HashMap::new();
            for (i, arg) in args.iter().enumerate() {
                if i >= manifest.parameters.len() {
                    error!(
                        "Too many positional parameters for the step! ({})",
                        manifest.symbol
                    );
                    break;
                }
                let key = manifest.parameters[i].name.clone();
                kwargs.insert(key, arg.clone());
            }
            StepParameters::Keyword(kwargs)
        }
    }
}

/**
 * The run method is the "core" of the agent which will run a series of steps
 * passed in.
 *
 * Currently it is very simple and primitive
 */
pub fn run(
    steps_dir: &str,
    steps: &Vec<Step>,
    pipeline: Uuid,
    controller: Option<Receiver<control::Request>>,
) -> std::io::Result<Status> {
    let manifests = load_manifests_for(steps_dir, steps)?;

    // XXX: hacks
    let mut endpoints = HashMap::new();
    endpoints.insert("objects".to_string(), object_endpoint_for(&pipeline));

    // Now that things are valid and collected, let's executed
    for step in steps.iter() {
        if let Some(ref ctl) = controller {
            while !ctl.is_empty() {
                if let Ok(msg) = ctl.try_recv() {
                    debug!("Processing control message in runloop: {:#?}", msg);
                    match msg {
                        control::Request::Terminate => {
                            // TODO: this needs to halt the entire pipeline, not just what is
                            // executing on this agent
                            info!("Runloop has been asked to terminate, exiting");
                            return Ok(Status::Failed);
                        }
                    }
                }
            }
        }
        if let Some(runner) = manifests.get(&step.symbol) {
            let entrypoint = runner.path.join(&runner.manifest.entrypoint.path);

            let mut file = NamedTempFile::new()?;

            let cache = match runner.manifest.cache {
                true => {
                    if let Ok(dir) = std::env::var("CACHES_DIR") {
                        Some(PathBuf::from(dir))
                    } else {
                        None
                    }
                }
                false => None,
            };

            // TODO: This is going to be wrong on nested steps
            let sock = control::agent_socket(&pipeline);
            let configuration = step::Configuration {
                pipeline: pipeline,
                uuid: step.uuid,
                cache: cache,
                ipc: sock,
                endpoints: endpoints.clone(),
            };
            let invocation: step::Invocation<StepParameters> = step::Invocation {
                configuration,
                parameters: positional_to_keyword(&step.parameters, &runner.manifest),
            };

            serde_json::to_writer(&mut file, &invocation)
                .expect("Failed to write temporary file for script");

            use os_pipe::pipe;
            use std::io::{BufRead, BufReader};
            let mut cmd = Command::new(entrypoint);
            cmd.arg(file.path());
            let (reader, writer) = pipe().unwrap();
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

            let status = handle.wait()?;

            let log = Log::StepEnd {
                symbol: step.symbol.clone(),
                uuid: step.uuid,
            };

            println!("{:?}", log);

            if !status.success() {
                info!("Step was not successful, exiting the runloop");
                // TODO: this needs to halt the entire pipeline, not just what is executing on this
                // agent
                return Ok(Status::Failed);
            }
        }
    }

    Ok(Status::Successful)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_manifests_invalid_dir() {
        let manifests = load_manifests_for("Cargo.toml", &vec![]);
        assert!(manifests.is_err());
    }

    #[test]
    fn load_manifests_empty_dir() {
        let manifests = load_manifests_for("src", &vec![]).expect("Failed to look into .git?");
        assert_eq!(manifests.len(), 0);
    }

    #[test]
    fn load_manifests_stdlib() {
        let params = serde_json::Value::Null;
        let step = Step {
            symbol: "echo".to_string(),
            uuid: otto_models::generate_uuid(),
            context: otto_models::generate_uuid(),
            parameters: StepParameters::Positional(vec![params]),
        };
        let manifests =
            load_manifests_for("../../stdlib", &vec![step]).expect("Failed to look into stdlib?");
        assert!(manifests.len() > 0);
    }

    #[test]
    fn pos_to_keyword() {
        use serde_json::Value;
        let arg = Value::String("ps".to_string());
        let parameters = StepParameters::Positional(vec![arg.clone()]);
        let manifests = load_manifests_for_symbols("../../stdlib", vec!["sh".to_string()])
            .expect("Failed to look into stdlib?");
        let loaded = manifests.get("sh").expect("Must have a `sh` manifest");

        let kwargs = positional_to_keyword(&parameters, &loaded.manifest);
        match kwargs {
            StepParameters::Positional(_) => assert!(false),
            StepParameters::Keyword(kw) => {
                assert_eq!(kw.get("script"), Some(&arg));
            }
        }
    }

    #[test]
    fn too_many_pos_to_keyword() {
        use serde_json::Value;
        let arg = Value::String("hi".to_string());
        let parameters = StepParameters::Positional(vec![arg.clone(), arg.clone()]);
        let manifests = load_manifests_for_symbols("../../stdlib", vec!["echo".to_string()])
            .expect("Failed to look into stdlib?");
        let loaded = manifests.get("echo").expect("Must have a `echo` manifest");

        let kwargs = positional_to_keyword(&parameters, &loaded.manifest);
        match kwargs {
            StepParameters::Positional(_) => assert!(false),
            StepParameters::Keyword(kw) => {
                assert_eq!(kw.get("message"), Some(&arg));
            }
        }
    }
}
