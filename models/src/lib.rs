use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use uuid::Uuid;

/**
 * A Pipeline contains the total configuration and steps for a single pipeline run
 */
#[derive(Clone, Debug, Deserialize)]
pub struct Pipeline {
    #[serde(default = "generate_uuid")]
    pub uuid: Uuid,
    pub contexts: Vec<Context>,
    pub steps: Vec<Step>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            uuid: generate_uuid(),
            contexts: vec![],
            steps: vec![],
        }
    }
}

/**
 * Possible statuses that a Pipeline can have
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Status {
    Successful = 0,
    Failed = 1,
    Aborted = 2,
    Unstable = 3,
}

/**
 * A context is some bucket of variables and configuration within a pipeline
 * this will most frequently be a "stage" in the conventional sense
 */
#[derive(Clone, Debug, Deserialize)]
pub struct Context {
    #[serde(default = "generate_uuid")]
    pub uuid: Uuid,
    pub name: String,
    pub environment: Option<HashMap<String, String>>,
}

impl Context {
    pub fn new(name: String) -> Self {
        Self {
            uuid: generate_uuid(),
            name,
            environment: None,
        }
    }
}

/**
 * A step is the smallest unit of execution for the pipeline
 */
#[derive(Clone, Debug, Deserialize)]
pub struct Step {
    #[serde(default = "generate_uuid")]
    pub uuid: Uuid,
    /// The uuid of the context to which this step is associated
    pub context: Uuid,
    pub symbol: String,
    pub parameters: Value,
}

impl Step {
    pub fn new(context: Uuid, symbol: String, parameters: Value) -> Self {
        Self {
            uuid: generate_uuid(),
            context, symbol, parameters,
        }
    }
}

/**
 * Generate a UUID v4 for use in structs, etc
 */
pub fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

#[cfg(test)]
mod tests {
}
