extern crate hyper;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use hyper::Client;
use hyper::rt::{self, Future, Stream};

mod manifest;
use manifest::Manifest;

fn main() {
    println!("Starting agent");
    rt::run(rt::lazy(|| {
        let client = Client::new();
        let uri = "http://localhost:3030/v1/manifest/rusty".parse().unwrap();

        client
            .get(uri)
            .and_then(|res| {
                println!("Response: {}", res.status());
                res.into_body().concat2()
            })
            .from_err::<FetchError>()
            .and_then(|body| {
                let manifest: Manifest = serde_json::from_slice(&body)?;
                println!("okie doke {}", manifest.agent);
                Ok(())
            })
            .map_err(|e| {
                match e {
                    FetchError::Http(e) => eprintln!("http error: {}", e),
                    FetchError::Json(e) => eprintln!("json parsing error: {}", e),
                }
            })
    }));
}


// Define a type so we can return multiple types of errors
enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}
