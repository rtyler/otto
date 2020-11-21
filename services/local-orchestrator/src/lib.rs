use log::*;
use std::path::PathBuf;
use tide::Request;

#[derive(Clone, Debug)]
pub struct State {
    pub upload_dir: PathBuf,
}

async fn put_object(req: Request<State>) -> tide::Result {
    use async_std::{fs::OpenOptions, io};
    let key = req.url().path();

    info!("Uploading: {:#?} into {:#?}", key, req.state().upload_dir);

    /*
     * A path will normally come in like /SomeFile.xlsx and Path::push will see
     * that as a new absolute file which doesn't _join_ but instead overwrites
     */
    let key = key.strip_prefix("/").unwrap_or(key);
    let fs_path = req.state().upload_dir.join(key);

    /*
     * In the case of nested keys, we need to create the layout on disk
     */
    if let Some(parent) = fs_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&fs_path)
        .await?;

    let bytes_written = io::copy(req, file).await?;

    tide::log::info!("file written", {
        bytes: bytes_written,
        path: fs_path.canonicalize()?.to_str()
    });

    Ok("{}".into())
}

async fn get_object(req: Request<State>) -> tide::Result {
    use tide::{Body, Response};

    let key = req.url().path();
    info!("Fetching: {:#?} from{:#?}", key, req.state().upload_dir);

    let key = key.strip_prefix("/").unwrap_or(key);
    let fs_path = req.state().upload_dir.join(key);

    if fs_path.exists() {
        Ok(Response::builder(200)
            .body(Body::from_file(&fs_path).await?)
            .build())
    } else {
        Err(tide::Error::from_str(404, "Failed to locate key"))
    }
}

pub fn app(mut upload_dir: PathBuf) -> tide::Server<State> {
    upload_dir = std::fs::canonicalize(upload_dir).expect("Unable to canonicalize the upload_dir");
    let state = State { upload_dir };
    let mut app = tide::with_state(state);
    app.at("/*").put(put_object);
    app.at("/*").get(get_object);
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app
}

#[cfg(test)]
mod tests {}
