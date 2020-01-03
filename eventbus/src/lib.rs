/**
 * The msg module contains common message definitions for serialization and deserialization of data
 * across the eventbus
 */
extern crate serde;
extern crate serde_json;

use actix::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;


/**
 * The Output enums are all meant to capture the types of messages which can be received from the
 * eventbus.
 *
 * Clients should be prepared to handle each of these messages coming over the channels they
 * subscribe to.
 */
#[derive(Serialize, Deserialize, Debug, Message)]
#[serde(tag = "output", rename_all = "camelCase")]
#[rtype(result = "()")]
pub enum Output {
    Heartbeat,
}

/**
 * The Input enums are all meant to capture the types of messages that can be send to the eventbus
 * as "inputs."
 *
 */
#[derive(Serialize, Deserialize, Debug, Message)]
#[serde(tag = "input", rename_all = "camelCase")]
#[rtype(result = "()")]
pub enum Input {
    /**
     * A Subscribe message must be sent for each channel the client wishes to subscribe to.
     *
     * These subscriptions are currently NOT durable. Once the client disconnects, subscriptions
     * will be cleared automatically
     */
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
    /**
     * The unsubscribe message can be sent if the client wishes to stop following a specific
     * channel, but remained connected and following others.
     */
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
    /**
     * The Publish message is the most common message clients should be sending
     *
     * The `payload` is an arbitrary bit of JSON, and is not typed
     */
    Publish {
        channel: String,
        payload: Value,
    },
}
