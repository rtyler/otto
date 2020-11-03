use async_std::{fs::OpenOptions, io};
use otto_agent::step::*;
use serde::Deserialize;
use std::io::{Error, ErrorKind};

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    name: String,
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let args = std::env::args().collect();
    let invoke: Invocation<Parameters> = invocation_from_args(&args).unwrap();

    let endpoint = invoke
        .configuration
        .endpoints
        .get("objects")
        .expect("Failed to get the `objects` endpoint!");

    let artifact_path = format!("{}/{}", endpoint.url, invoke.parameters.name);

    let response = surf::get(artifact_path)
        .await
        .expect("Failed to query object-store");

    if response.status() == 200 {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&invoke.parameters.name)
            .await?;
        io::copy(response, file).await?;
    } else {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Could not locate the artifact",
        ));
    }

    Ok(())
}
