/**
 * The msg module contains common message definitions for serialization and deserialization of data
 * across the eventbus
 */


use serde::{Deserialize, Serialize};
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "camelCase")]
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
