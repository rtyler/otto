/*
 * The local orchestrator doesn't do much
 */

#[async_std::main]
async fn main() -> std::io::Result<()> {
    use std::{env, net::TcpListener, os::unix::io::FromRawFd};
    tide::log::start();

    let app = tide::new();

    if let Some(fd) = env::var("LISTEN_FD").ok().and_then(|fd| fd.parse().ok()) {
        app.listen(unsafe { TcpListener::from_raw_fd(fd) }).await?;
    } else {
        app.listen("http://localhost:7673").await?;
    }
    Ok(())
}
