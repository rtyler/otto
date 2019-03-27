use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    #[serde(rename(deserialize = "self"))]
    pub agent   : String,
    pub services: HashMap<String, String>,
    pub ops     : Vec<Operation>,
}

#[derive(Deserialize, Debug)]
pub struct Operation {
    pub op_id: String,
    #[serde(rename(deserialize = "type"))]
    pub op_type : String,
    pub data    : serde_json::Map<String, serde_json::Value>,
}
