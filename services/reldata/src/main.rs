/*
 * The relational data service largely is meant to expose information from an underlying database
 */
use tide::Request;

async fn healthcheck(_req: Request<()>) -> tide::Result {
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

    if let Some(fd) = env::var("LISTEN_FD").ok().and_then(|fd| fd.parse().ok()) {
        app.listen(unsafe { TcpListener::from_raw_fd(fd) }).await?;
    } else {
        app.listen("http://localhost:7674").await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
