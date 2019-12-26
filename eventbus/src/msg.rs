/**
 * The msg module contains common message definitions for serialization and deserialization of data
 * across the eventbus
 */
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
enum CommandType {
    Subscribe,
    Unsubscribe,
}

#[derive(Serialize, Deserialize)]
struct Basic {
    command: CommandType,
    payload: Value,
}
