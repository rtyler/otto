use std::path::Path;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    use std::{env, net::TcpListener, os::unix::io::FromRawFd};
    tide::log::start();

    let upload_dir = std::env::var("OTTO_OBJECT_DIR").unwrap_or("tmp".to_string());
    let app = otto_objectstore::app(Path::new(&upload_dir).to_path_buf());

    if let Some(fd) = env::var("LISTEN_FD").ok().and_then(|fd| fd.parse().ok()) {
        app.listen(unsafe { TcpListener::from_raw_fd(fd) }).await?;
    } else {
        app.listen("http://localhost:7671").await?;
    }
    Ok(())
}
