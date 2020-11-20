/*
 * The git step does simple clones of git repositories
 */

use otto_agent::step::*;
use serde::Deserialize;
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

fn main() -> std::io::Result<()> {
    use std::path::Path;

    let args = std::env::args().collect();
    let invoke: Invocation<Parameters> =
        invocation_from_args(&args).expect("Failed to deserialize the invocation for the step");

    let clone_path = match invoke.parameters.into {
        Some(into) => into,
        None => {
            repo_from_url(&invoke.parameters.url).expect("Failed to determine local path to clone")
        }
    };

    println!("Clone!");
    let mut builder = git2::build::RepoBuilder::new();

    if let Some(branch) = &invoke.parameters.branch {
        builder.branch(&branch);
    }

    let _repo = match builder.clone(&invoke.parameters.url.into_string(), Path::new(&clone_path)) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

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
}
