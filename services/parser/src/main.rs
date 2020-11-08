/*
 * THe parser web service basically just takes in pipeline syntax and spits out the machine
 * readable (YAML) structures which other components in Otto can use.
 */

#[macro_use]
extern crate serde_json;

use log::*;
use otto_parser::*;
use tide::{Request, Response};

async fn parse(mut req: Request<()>) -> tide::Result {
    if let Ok(body) = req.body_string().await {
        let parsed = parse_pipeline_string(&body);

        match parsed {
            Err(e) => {
                error!("Failed to parse: {:#?}", e);
                return Ok(Response::builder(400)
                    .body(json!({
                        "variant" : "",
                        "location" : "",
                        "line" : 0,
                        "column" : 0
                    }))
                    .content_type("application/json")
                    .build()
                );
            },
            Ok(pipeline) => {
                return Ok(Response::builder(200)
                            .body(json!({"meta": {}}))
                            .content_type("application/json")
                            .build());
            }
        }
    }

    // Setting the content type manually since body_string? won't return one
    Ok(Response::builder(422).content_type("application/json").build())
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

