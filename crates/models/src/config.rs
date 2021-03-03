/*
 * The config module is responsible for deserializing common configuration stored in
 * PREFIX/etc/otto
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/**
 * Struct representing the main configuration file for Otto, e.g. PREFIX/etc/otto/otto.yml
 *
 * ```rust
 * use otto_models::config::Otto;
 * let yaml = r#"
 * ---
 * services:
 *   dashboard:
 *     host: 'localhost'
 *     port: 7670
 *         "#;
 * let otto: Otto = serde_yaml::from_str(&yaml).expect("Failed to deserialize");
 * assert_eq!(otto.services.len(), 1);
 * ```
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Otto {
    pub services: HashMap<String, Service>,
}

/**
 * Service definition within the main otto configuration
 *
 * All fields are currently required to be present in the configuration
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Service {
    pub host: String,
    pub port: u64,
}

/** Project definition for Otto.
 *
 * This struct maps the configuration for the yaml files typically expected to be found inside of
 * PREFIX/etc/otto/projects.d/
 *
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub source: SourceMap,
    #[serde(default = "default_pipeline")]
    pub pipeline: Pipeline,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceMap {
    pub url: String,
    #[serde(default = "default_refspec")]
    pub refspec: String,
}

/**
 * The default refspec is *, so matching all refs
 */
fn default_refspec() -> String {
    String::from("*")
}

/**
 * The default pipeline configuration for a project will also point to the local ./Ottofile
 */
fn default_pipeline() -> Pipeline {
    Pipeline {
        path: Some(PathBuf::from("./Ottofile")),
        inline: None,
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pipeline {
    pub path: Option<PathBuf>,
    pub inline: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser_otto_yml() {
        let yaml = r#"
---
services:
  dashboard:
    host: 'localhost'
    port: 7670
        "#;
        let otto: Otto = serde_yaml::from_str(&yaml).expect("Failed to deserialize");
        assert!(otto.services.contains_key("dashboard"));
        let dashboard = otto.services.get("dashboard").unwrap();
        assert_eq!(dashboard.port, 7670);
    }

    #[test]
    fn deser_project_yml() {
        let yaml = r#"
---
title: 'Hello World'
description: |
  The Hello World project exists as a simple demonstrate loading a project into Otto
source:
  url: 'https://github.com/rtyler/hello-gem.git'
"#;
        let project: Project = serde_yaml::from_str(&yaml).expect("Failed to deser project");
        assert_eq!(project.title, "Hello World");
        assert_eq!(project.source.refspec, "*");
    }

    #[test]
    fn test_default_refspec() {
        assert_eq!(default_refspec(), "*");
    }

    #[test]
    fn test_default_pipeline() {
        let pipeline = default_pipeline();
        assert!(pipeline.path.is_some());
        let path = pipeline.path.unwrap();
        assert_eq!(path, PathBuf::from("./Ottofile"));
    }

    /*
     * This is a failure, a declared pipeline: must have contents
     */
    #[test]
    fn deser_project_w_empty_pipeline() {
        let yaml = r#"
---
title: 'Hello World'
description: |
  The Hello World project exists as a simple demonstrate loading a project into Otto
source:
  url: 'https://github.com/rtyler/hello-gem.git'
pipeline:
"#;
        let rc = serde_yaml::from_str::<Project>(&yaml);
        assert!(rc.is_err(), "An empty pipeline: is an error condition");
    }

    #[test]
    fn deser_project_w_inline_pipeline() {
        let yaml = r#"
---
title: 'Hello World'
description: |
  The Hello World project exists as a simple demonstrate loading a project into Otto
source:
  url: 'https://github.com/rtyler/hello-gem.git'
pipeline:
  inline: 'this is just a buffer not a real pipeline'
"#;
        let project: Project = serde_yaml::from_str(&yaml).expect("Failed to deser project");
        assert!(project.pipeline.inline.is_some());
    }
}
