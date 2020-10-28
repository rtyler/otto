/*
 * This module contains a lot of the code to support steps and is expected to
 * be included by many Rust steps to re-use common code.
 */

use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

/**
 * The Invocation struct describes the structure of the invocation file which
 * will be passed to the steps as the first parameter on the command line.
 *
 * Steps should define their own parameter structs which can be passed in as a
 * generic parameter.
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invocation<P> {
    /// Configuration contains general configuration for the step to utilize
    pub configuration: Configuration,
    /// Parameters are to be a step-defined type
    pub parameters: P,
}

/**
 * The Configuration struct will carry important information about the Otto
 * system into the step, such as the IPC path or endpoints where it can put data
 * as needed
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub ipc: std::path::PathBuf,
    pub endpoints: HashMap<String, Endpoint>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Endpoint {
    pub url: Url,
}

/**
 * This function will handle parsing the command line arguments passed to the step
 * and return the desired Invocation struct
 */
pub fn invocation_from_args<P: serde::de::DeserializeOwned>(
    args: &Vec<String>,
) -> Result<Invocation<P>, std::io::Error> {
    use std::fs::File;
    use std::io::{Error, ErrorKind};

    if args.len() != 2 {
        error!("A step should only be invoked with a single argument: the invocation file path");
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Step must only be invoked with a single argument",
        ));
    }

    let file = File::open(&args[1])?;

    match serde_yaml::from_reader::<File, Invocation<P>>(file) {
        Err(e) => {
            error!("Failed to deserialize invocation file: {:#?}", e);
            Err(Error::new(ErrorKind::InvalidData, e))
        }
        Ok(invoke) => Ok(invoke),
    }
}
