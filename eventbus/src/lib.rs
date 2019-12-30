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

#[derive(Serialize, Deserialize, Debug, Message)]
#[serde(tag = "command", rename_all = "camelCase")]
#[rtype(result = "()")]
pub enum Command {
    Heartbeat,
    Subscribe {
        /**
         * The client's UUID
         */
        client: String,
        /**
         * The channel the client wishes to subscribe to
         */
        channel: String,
    },
    Unsubscribe {
        /**
         * The client's UUID
         */
        client: String,
        /**
         * The channel the client wishes to unsubscribe from
         */
        channel: String,
    },
    Publish {
        channel: String,
        payload: Value,
    },
}
