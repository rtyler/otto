use std::path::Path;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let upload_dir = std::env::var("OTTO_OBJECT_DIR").unwrap_or("tmp".to_string());
    otto_objectstore::app(Path::new(&upload_dir).to_path_buf())
        .listen("127.0.0.1:7671")
        .await?;
    Ok(())
}
