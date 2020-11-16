/*
 * The git step does simple clones of git repositories
 */

use git2::Repository;
use otto_agent::step::*;
use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    url: Url,
    r#ref: Option<String>,
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
    let args = std::env::args().collect();
    let invoke: Invocation<Parameters> = invocation_from_args(&args)
        .expect("Failed to deserialize the invocation for the step");

    if let Some(path) = repo_from_url(&invoke.parameters.url) {
        let _repo = match Repository::clone(&invoke.parameters.url.into_string(), path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
    }
    else {
        println!("Failed to determine the right local path to clone the repository into");
        std::process::exit(1);
    }
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
