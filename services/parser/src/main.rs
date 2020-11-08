/*
 * THe parser web service basically just takes in pipeline syntax and spits out the machine
 * readable (YAML) structures which other components in Otto can use.
 */

use otto_parser::*;
use tide::{Request, Response};

async fn parse(mut req: Request<()>) -> tide::Result {
    let buffer = req.body_string().await?;

    let parsed = parse_pipeline_string(&buffer);

    match parsed {
        Err(e) => {
            let resp = Response::builder(400)
                .body("{}")
                .content_type("application/json")
                .build();
            return Ok(resp);
        },
        Ok(pipeline) => {
            let resp = Response::builder(200)
                        .body(r#"{"meta" : {}}"#)
                        .content_type("application/json")
                        .build();
            return Ok(resp);
        }
    }
}


#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    use std::{env, net::TcpListener, os::unix::io::FromRawFd};
    tide::log::start();
    let mut app = tide::new();
    app.at("/v1/parse").post(parse);

    if let Some(fd) = env::var("LISTEN_FD").ok().and_then(|fd| fd.parse().ok()) {
        app.listen(unsafe { TcpListener::from_raw_fd(fd) }).await?;
    }
    else {
        app.listen("http://localhost:7672").await?;
    }
    Ok(())
}

