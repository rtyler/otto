/*
 * The git step does simple clones of git repositories
 */

use otto_agent::step::*;
use serde::Deserialize;
use std::path::PathBuf;
use url::Url;

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    url: Url,
    branch: Option<String>,
    into: Option<String>,
}

/**
 * This funciton is a simple function which will just tease out the local directory
 * path which should be used for cloning this repository URL
 */
fn repo_from_url(repo_url: &Url) -> Option<String> {
    if let Some(segments) = repo_url.path_segments() {
        if let Some(last) = segments.last() {
            return Some(last.replace(".git", ""));
        }
    }
    None
}

/**
 * Generate the reference repo path from the given Url
 */
fn locate_reference_for(url: &Url, cache_dir: &PathBuf) -> PathBuf {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(url.as_str());
    let result = hasher.finalize();
    cache_dir.join(format!("{:x}", result))
}

/**
 * Clone a Git repository
 */
fn clone(
    repo: String,
    into: &PathBuf,
    branch: Option<String>,
    bare: Option<bool>,
) -> std::io::Result<()> {
    let mut builder = git2::build::RepoBuilder::new();

    if let Some(branch) = branch {
        builder.branch(&branch);
    }

    if let Some(bare) = bare {
        // https://github.com/rust-lang/git2-rs/issues/521
        builder
            .bare(bare)
            .remote_create(|repo, name, url| repo.remote_with_fetch(name, url, "+refs/*:refs/*"));
    }

    println!("Cloning {} into {:?}", repo, into);

    let _repo = match builder.clone(&repo, into) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone {} to {:?}: {}", repo, into, e),
    };
    Ok(())
}

/**
 * Fetch all remotes in the given repository
 */
fn fetch(repo_path: &PathBuf, refs: Vec<String>, bare: bool) {
    println!("Fetching updates for {:?} - {:?}", repo_path, refs);
    let repo = match bare {
        true => git2::Repository::open_bare(&repo_path).expect("Failed to open repo"),
        false => git2::Repository::open(&repo_path).expect("Failed to open repo"),
    };

    if let Ok(remotes) = repo.remotes() {
        for remote in remotes.iter() {
            if let Ok(mut remote) = repo.find_remote(remote.unwrap()) {
                remote.fetch(&refs, None, None).expect("Failed to fetch");
            }
        }
    }
}

/**
 * Return the String of the URL of the repo that should be relied upon for cloning
 */
fn reference_or_upstream_repo(invoke: &Invocation<Parameters>) -> String {
    let url = &invoke.parameters.url;

    if let Some(cache) = &invoke.configuration.cache {
        /*
         * When a cache directory is present, the step should create a new cached clone
         * for this repo, or update the existing one and return the path
         */
        let ref_repo = locate_reference_for(url, cache);

        if ref_repo.as_path().is_dir() {
            let refs = match &invoke.parameters.branch {
                Some(branch) => vec![branch.clone()],
                None => vec![],
            };
            fetch(&ref_repo, refs, true);
        } else {
            clone(
                url.clone().into_string(),
                &ref_repo,
                invoke.parameters.branch.clone(),
                Some(true),
            );
        }
        ref_repo.as_path().to_string_lossy().to_string()
    } else {
        /*
         * In the cases where the cache directory isn't known, the step is just
         * going to have to clone the source repo
         */
        url.clone().into_string()
    }
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect();
    let invoke: Invocation<Parameters> =
        invocation_from_args(&args).expect("Failed to deserialize the invocation for the step");

    let repo_url = reference_or_upstream_repo(&invoke);
    let clone_path = match invoke.parameters.into {
        Some(into) => into,
        None => {
            repo_from_url(&invoke.parameters.url).expect("Failed to determine local path to clone")
        }
    };

    clone(
        repo_url,
        &PathBuf::from(clone_path),
        invoke.parameters.branch,
        None,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_simple_url() {
        let u = Url::parse("https://example.com/repo.git").expect("Failed to parse");
        assert_eq!(repo_from_url(&u).unwrap(), "repo");
    }

    #[test]
    fn test_simple_url_no_dot_git() {
        let u = Url::parse("https://example.com/repo").expect("Failed to parse");
        assert_eq!(repo_from_url(&u).unwrap(), "repo");
    }

    #[test]
    fn test_location_reference_for() {
        use std::path::PathBuf;
        let pb = PathBuf::from("/tmp/");
        let url = Url::parse("https://example.com").expect("Failed to parse url");
        let result = locate_reference_for(&url, &pb);
        assert_eq!(
            PathBuf::from("/tmp/0f115db062b7c0dd030b16878c99dea5c354b49dc37b38eb8846179c7783e9d7"),
            result
        );
    }
}
