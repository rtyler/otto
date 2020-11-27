use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub use serde_json::Value;
pub mod osp;

/**
 * A Pipeline contains the total configuration and steps for a single pipeline run
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pipeline {
    #[serde(default = "generate_uuid")]
    pub uuid: Uuid,
    pub batches: Vec<Batch>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            uuid: generate_uuid(),
            batches: vec![],
        }
    }
}

/**
 * A batch is a collection of contexts that should be executed in a given mode.
 *
 * This structure basically allows for Otto to execute batches of contexts in parallel, or in various
 * other flows.
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Batch {
    pub mode: BatchMode,
    pub contexts: Vec<Context>,
}

impl Default for Batch {
    fn default() -> Self {
        Self {
            mode: BatchMode::Linear,
            contexts: vec![],
        }
    }
}

/**
 * The mode in which an orchestrator should execute the batch
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BatchMode {
    /// Each context should be executed in order
    Linear,
    /// Each context should be executed in parallel completely independent of each other
    Parallel,
    /// Each context should be executed in parallel but all should cancel on the first failure
    Fanout,
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
 * this will most frequently be a "stage" in the conventional pipeline
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    #[serde(default = "generate_uuid")]
    pub uuid: Uuid,
    pub properties: HashMap<String, String>,
    pub environment: Option<HashMap<String, String>>,
    pub steps: Vec<Step>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            uuid: generate_uuid(),
            properties: HashMap::default(),
            environment: None,
            steps: vec![],
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
        {"symbol":"sh",
        "uuid":"5599cffb-f23a-4e0f-a0b9-f74654641b2b",
        "context":"3ce1f6fb-79ca-4564-a47e-98265f53ef7f",
        "parameters" : [
            "ls -lah | tail -n 5"
        ]}"#;
        let step = serde_json::from_str::<Step>(&buf).expect("Failed to deserialize");

        assert_eq!(step.symbol, "sh");
    }

    /*
     * https://github.com/rtyler/otto/issues/42
     */
    #[test]
    fn deserialize_positional_issue_42() {
        #[derive(Clone, Debug, Deserialize, Serialize)]
        struct Pipeline {
            pipeline: String,
            steps: Vec<Step>,
        }
        let buf = r#"{"pipeline":"fdbebdcf-ad5c-49e5-890f-aef294b476c5","steps":[{"uuid":"f619073f-4129-4d30-a94f-f61af164a6d8","context":"fdbebdcf-ad5c-49e5-890f-aef294b476c5","symbol":"sh","parameters":["pwd"]}]}"#;
        let pipeline = serde_json::from_str::<Pipeline>(&buf).expect("Failed to deserialize");

        assert_eq!(pipeline.steps[0].symbol, "sh");
    }

    #[test]
    fn deserialize_kwargs() {
        let buf = r#"
        {"symbol":"sh",
        "uuid":"5599cffb-f23a-4e0f-a0b9-f74654641b2b",
        "context":"3ce1f6fb-79ca-4564-a47e-98265f53ef7f",
        "parameters" : {
            "script" : "ls -lah | tail -n 5"
        }}"#;
        let step = serde_json::from_str::<Step>(&buf).expect("Failed to deserialize");

        assert_eq!(step.symbol, "sh");
    }
}
