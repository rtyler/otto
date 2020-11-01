/*
 * The archive step will store artifacts on the server associated with the
 * running pipeline
 */

use flate2::write::GzEncoder;
use flate2::Compression;
use glob::glob;
use ottoagent::step::*;
use serde::Deserialize;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    artifacts: String,
    /// The name to be used when storing the artifact
    name: Option<String>,
    #[serde(rename = "followSymlinks")]
    follow_symlinks: Option<bool>,
}

/**
 * Will return a vec of PathBufs that the specified pattern matches
 *
 * If the pattern is an invalid glob, there will be a panic and the step will exit
 */
fn artifact_matches(pattern: &str) -> Vec<PathBuf> {
    glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(std::result::Result::ok)
        .map(|p| p)
        .collect()
}

/**
 * This function will create a tarball based on the given paths
 */
fn create_tarball(output: &str, paths: &Vec<PathBuf>) -> std::io::Result<PathBuf> {
    let output = format!("{}.tar.gz", output);
    let path = Path::new(&output);
    let tar_gz = File::create(&output)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());

    let mut tar = tar::Builder::new(enc);
    for path in paths.iter() {
        if path.is_dir() {
            tar.append_dir_all(".", path)
                .expect(&format!("Failed to add {:#?} to the tarball", path));
        } else {
            tar.append_path(path)
                .expect(&format!("Failed to add {:#?} to the tarball", path));
        }
    }
    tar.finish()?;

    Ok(path.to_path_buf())
}

/**
 * Determines whether the path provided actually lives inside of current_dir()
 *
 * Will return false if a path traversal attack is being attempted
 */
fn is_child_path(path: &Path) -> bool {
    let current =
        std::env::current_dir().expect("Failed to get current_dir, cannot safely execute");
    let current_components = current.components().collect::<Vec<_>>();

    if let Ok(canonical) = path.canonicalize() {
        let components = canonical.components().collect::<Vec<_>>();

        // This clearly isn't a subdirectory or file of our current root
        if components.len() < current_components.len() {
            return false;
        }

        for (index, part) in current_components.iter().enumerate() {
            if components[index] != *part {
                return false;
            }
        }
        // If we have more components than current_components but they all have a common root, then
        // that's fine
        return true;
    }

    // Default to false, basically if this cannot prove it's not a path traversal
    // then assume it is.
    return false;
}

/**
 * Actually archive the named file into the object store
 *
 * The path should be the path to a single file, or a generated tarball
 */
async fn archive(path: &PathBuf, endpoint: &Endpoint) -> std::io::Result<()> {
    use surf::Body;

    println!("??? Archiving {:?} to {:?}", path, endpoint);
    surf::put(format!("{}/{}", endpoint.url, path.to_string_lossy()))
        .body(Body::from_file(path).await?)
        .await
        .expect("Failed to upload artifact!");
    Ok(())
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let args = std::env::args().collect();
    let invoke: Invocation<Parameters> = invocation_from_args(&args).unwrap();

    let endpoint = invoke
        .configuration
        .endpoints
        .get("objects")
        .expect("Failed to get the `objects` endpoint!");

    let artifacts = artifact_matches(&invoke.parameters.artifacts);

    match artifacts.len() {
        0 => {
            panic!(
                "The `archive` step was given a pattern ({}) which doesn't match any files",
                invoke.parameters.artifacts
            );
        }
        1 => {
            // no tarball, unless it's a directory
            let file = &artifacts[0];
            let name = match invoke.parameters.name {
                None => file
                    .as_path()
                    .file_name()
                    .expect("Failed to determine the file name for the archive")
                    .to_string_lossy()
                    .into_owned(),
                Some(name) => name,
            };

            // No archiving /etc/passwd you silly goose
            if !is_child_path(&file) {
                panic!("the archive step cannot archive paths outside of its current directory");
            }

            if file.is_dir() {
                create_tarball(&name, &artifacts);
            } else {
                archive(file, &endpoint).await?;
            }
        }
        _ => {
            let name = match invoke.parameters.name {
                None => "archive".to_string(),
                Some(name) => name,
            };

            match create_tarball(&name, &artifacts) {
                Err(_e) => {
                    // TODO handle
                }
                Ok(file) => {
                    archive(&file, &endpoint).await?;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

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

    #[test]
    fn is_child_path_attack() {
        let path = Path::new("src/../../");
        assert!(is_child_path(&path) == false);
    }

    #[test]
    fn is_child_path_legit() {
        let path = Path::new("src");
        assert!(is_child_path(&path));
    }
}
