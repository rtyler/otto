extern crate hyper;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


extern crate tempfile;
use tempfile::NamedTempFile;
use std::io::Write;

use std::process::Command;

use hyper::Client;
use hyper::rt::{self, Future, Stream};

mod manifest;
use manifest::{Manifest, Operation};

fn run_process(op: Operation) {
    let mut tmpfile = NamedTempFile::new().unwrap();
    write!(tmpfile, "#!/bin/sh\n{}", op.data["script"].as_str().unwrap()).unwrap();
    let mut proc = Command::new("sh");
    proc.arg(tmpfile.path().to_str().unwrap());
    let output = proc.output().expect("Failed to run subprocess");
    println!("executed ({}):\n{}", output.status, String::from_utf8(output.stdout).unwrap());
}

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
                println!("okie doke {}, let's get started", manifest.agent);
                for operation in manifest.ops {
                    println!("op: {}", operation.op_type);
                    if operation.op_type == "RUNPROC" {
                        run_process(operation);
                    }
                }
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
