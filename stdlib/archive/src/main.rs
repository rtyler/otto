/*
 * The archive step will store artifacts on the server associated with the
 * running pipeline
 */

use glob::glob;
use serde::Deserialize;
use std::path::PathBuf;
use ottoagent::step::*;

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    artifacts: String,
    #[serde(rename = "followSymlinks")]
    follow_symlinks: Option<bool>,
}

/**
 * Will return a vec of PathBufs that the specified pattern matches
 *
 * If the pattern is an invalid glob, there will be a panic and the step will exit
 */
fn artifact_matches(pattern: &str) -> Vec<PathBuf> {
    glob(pattern).expect("Failed to read glob pattern")
        .filter_map(std::result::Result::ok)
        .map( |p| p ).collect()
}

/**
 * This function will create a tarball based on the given paths
 */
fn create_tarball(paths: Vec<PathBuf>) -> std::io::Result<()> {
    Ok(())
}

/**
 * Actually archive the named file into the object store
 *
 * The path should be the path to a single file, or a generated tarball
 */
fn archive(path: &PathBuf) -> std::io::Result<()> {
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect();
    let invoke: Invocation<Parameters> = invocation_from_args(&args).unwrap();

    let artifacts = artifact_matches(&invoke.parameters.artifacts);

    match artifacts.len() {
        0 => {
            panic!("The `archive` step was given a pattern ({}) which doesn't match any files", invoke.parameters.artifacts);
        },
        1 => {
            // no tarball, unless it's a directory
            let file = &artifacts[0];
            if file.is_dir() {
                create_tarball(artifacts);
            }
            else {
                archive(file);
            }
        },
        _ => {
            match create_tarball(artifacts) {
                Err(e) => {
                    // TODO handle
                },
                Ok(file) => {
                    //archive(file);
                }
            }
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_matches_empty_pattern() {
        let paths = artifact_matches("/this/will/never/exist");
        assert_eq!(paths.len(), 0);
    }

    #[test]
    fn artifact_matches_single_file() {
        let paths = artifact_matches("/dev/null");
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn artifact_matches_wildcard() {
        let paths = artifact_matches("*");
        assert!(paths.len() > 1);
    }
}
