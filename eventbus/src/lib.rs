/**
 * The msg module contains common message definitions for serialization and deserialization of data
 * across the eventbus
 */
extern crate serde;
extern crate serde_json;

use actix::Message;
use serde::{Deserialize, Serialize};

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
    }
}
