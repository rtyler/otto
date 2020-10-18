

use serde::Deserialize;
use std::collections::HashMap;


#[derive(Clone, Debug, Deserialize)]
struct Pipeline {
    steps: Vec<Step>,
}

#[derive(Clone, Debug, Deserialize)]
struct Step {
    symbol: String,
    parameters: HashMap<String, String>,
}


fn main() {
    println!("Hello, world!");
}
