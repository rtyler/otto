/*
 * The local orchestrator doesn't do much
 */
use async_std::task;
use log::*;
use serde::Deserialize;
use tide::Request;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
struct RunWorkload {
    pipeline: Uuid,
    contexts: Vec<otto_models::Context>,
}

/**
 * This function is the core of the local-orchestrator in that it takes a
 * context and will spawn an agent to run it.
 *
 */
fn run_context(pipeline: &Uuid, ctx: &otto_models::Context) -> std::io::Result<bool> {
    use std::io::{Error, ErrorKind};
    use std::process::Command;
    use tempfile::NamedTempFile;

    let mut file = NamedTempFile::new()?;
    let invocation = otto_agent::Invocation {
        pipeline: *pipeline,
        steps: ctx.steps.clone(),
    };

    println!("{}", serde_json::to_string(&invocation).unwrap());

    if let Err(failure) = serde_json::to_writer(&mut file, &invocation) {
        error!("Failed to write temporary file for agent: {:#?}", failure);
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Could not write temporary file",
        ));
    }

    if let Ok(output) = Command::new("otto-agent").arg(file.path()).output() {
        use std::io::{stdout, Write};
        stdout().write(&output.stdout);

        return Ok(output.status.success());
    } else {
        // TODO
        error!("Failed to run agent");
    }

    Ok(false)
}

async fn healthcheck(_req: Request<()>) -> tide::Result {
    Ok(tide::Response::builder(200)
        .body("{}")
        .content_type("application/json")
        .build())
}

async fn run_workload(mut req: Request<()>) -> tide::Result {
    let run: RunWorkload = req.body_json().await?;
    debug!("Received RunWorkload: {:?}", run);

    task::spawn(async move {
        for ctx in run.contexts.iter() {
            match run_context(&run.pipeline, ctx) {
                Ok(success) => {
                    if !success {
                        return;
                    }
                    debug!("Context succeeded, continuing");
                }
                Err(_) => {
                    return;
                }
            }
        }
    });

    Ok(tide::Response::builder(200)
        .body("{}")
        .content_type("application/json")
        .build())
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    use std::{env, net::TcpListener, os::unix::io::FromRawFd};
    pretty_env_logger::init();

    let mut app = tide::new();
    app.at("/health").get(healthcheck);
    app.at("/v1/run").post(run_workload);

    if let Some(fd) = env::var("LISTEN_FD").ok().and_then(|fd| fd.parse().ok()) {
        app.listen(unsafe { TcpListener::from_raw_fd(fd) }).await?;
    } else {
        app.listen("http://localhost:7673").await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
