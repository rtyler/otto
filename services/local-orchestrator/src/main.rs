/*
 * The local orchestrator doesn't do much
 */
use log::*;
use serde::Deserialize;
use tide::Request;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
struct RunWorkload {
    pipeline: Uuid,
    contexts: Vec<otto_models::Context>,
}

async fn healthcheck(_req: Request<()>) -> tide::Result {
    Ok(tide::Response::builder(200)
        .body("{}")
        .content_type("application/json")
        .build())
}

async fn run_workload(mut req: Request<()>) -> tide::Result {
    let run: RunWorkload =  req.body_json().await?;
    debug!("Received RunWorkload: {:#?}", run);

    // TODO: do something actually useful :D

    Ok(tide::Response::builder(200)
        .body("{}")
        .content_type("application/json")
        .build())
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    use std::{env, net::TcpListener, os::unix::io::FromRawFd};
    tide::log::start();

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
