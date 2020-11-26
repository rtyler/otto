use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub symbol: String,
    #[serde(default = "default_false")]
    pub cache: bool,
    pub description: String,
    pub includes: Vec<Include>,
    pub entrypoint: Entrypoint,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Include {
    pub name: String,
    #[serde(default = "default_false")]
    pub flatten: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entrypoint {
    pub path: std::path::PathBuf,
    #[serde(default = "default_false")]
    pub multiarch: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameter {
    pub name: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub p_type: ParameterType,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ParameterType {
    #[serde(rename = "string")]
    StringParameter,
    #[serde(rename = "boolean")]
    BoolParameter,
    #[serde(rename = "block")]
    BlockParameter,
}

/** Simple function for serde defaults */
fn default_false() -> bool {
    false
}

#[cfg(test)]
mod tests {}
