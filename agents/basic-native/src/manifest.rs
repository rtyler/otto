use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    #[serde(rename(deserialize = "self"))]
    pub agent   : String,
    services: HashMap<String, String>,
    ops     : Vec<Operation>,
}

#[derive(Deserialize, Debug)]
struct Operation {
    op_id: String,
    #[serde(rename(deserialize = "type"))]
    op_type : String,
    data    : serde_json::Map<String, serde_json::Value>,
}
