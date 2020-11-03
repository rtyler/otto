use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use uuid::Uuid;

/**
 * A Pipeline contains the total configuration and steps for a single pipeline run
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
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
 *
 * Each of the statuses are mapped to an i32 value such that shell exit codes can easily be used to
 * set the pipeline status.
 *
 * For example, if a step's invocation returns a 3 exit code, then the agent should automatically
 * know to the set the pipeline status to Unstable
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    #[serde(default = "generate_uuid")]
    pub uuid: Uuid,
    pub properties: HashMap<String, String>,
    pub environment: Option<HashMap<String, String>>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            uuid: generate_uuid(),
            properties: HashMap::default(),
            environment: None,
        }
    }
}

/**
 * A step is the smallest unit of execution for the pipeline
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Step {
    #[serde(default = "generate_uuid")]
    pub uuid: Uuid,
    /// The uuid of the context to which this step is associated
    pub context: Uuid,
    pub symbol: String,
    pub parameters: StepParameters,
}

impl Step {
    pub fn new(context: Uuid, symbol: String, parameters: StepParameters) -> Self {
        Self {
            uuid: generate_uuid(),
            context,
            symbol,
            parameters,
        }
    }
}

/**
 * The StepParameters enum helps map map both positional and keyword
 * parameters for a given step.
 *
 * Steps should all support either positional or keyword parameters.
 *
 * Step libraries define their parameters in an array, so the positional parameters are expected to
 * map into those directly, meaning there are no optional parameters.
 *
 * When using keyword parameters, users should be able to pick and choose which parameters to
 * define.
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StepParameters {
    Positional(Vec<Value>),
    Keyword(HashMap<String, Value>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MockStep {
    symbol: String,
    parameters: StepParameters,
}

/**
 * Generate a UUID v4 for use in structs, etc
 *
 * ```rust
 * # use otto_models::generate_uuid;
 * let uuid = generate_uuid();
 * assert_eq!(false, uuid.is_nil());
 * ```
 */
pub fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_positional() {
        let buf = r#"
    symbol: sh
    uuid: '5599cffb-f23a-4e0f-a0b9-f74654641b2b'
    context: '3ce1f6fb-79ca-4564-a47e-98265f53ef7f'
    parameters:
      - 'ls -lah | tail -n 5'"#;
        let step = serde_yaml::from_str::<Step>(&buf).expect("Failed to deserialize");

        assert_eq!(step.symbol, "sh");
    }

    #[test]
    fn deserialize_kwargs() {
        let buf = r#"
    symbol: sh
    uuid: '5599cffb-f23a-4e0f-a0b9-f74654641b2b'
    context: '3ce1f6fb-79ca-4564-a47e-98265f53ef7f'
    parameters:
      script: 'ls -lah | tail -n 5'"#;
        let step = serde_yaml::from_str::<Step>(&buf).expect("Failed to deserialize");

        assert_eq!(step.symbol, "sh");
    }
}
