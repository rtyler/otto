/**
 * The msg module contains common message definitions for serialization and deserialization of data
 * across the eventbus
 */
extern crate serde;
extern crate serde_json;

use actix::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod bus;
pub mod client;

#[derive(Serialize, Deserialize)]
pub enum CommandType {
    Subscribe,
    Unsubscribe,
    Heartbeat,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct Basic {
    pub command: CommandType,
    /**
     * The payload can be of any `Value` type from the serde_json crate
     */
    pub payload: Value,
}
