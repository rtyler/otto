#![allow(unused_imports)]
#![deny(unsafe_code)]
/**
 * The Otto Eventbus module contains public interfaces for sending messages, constructing clients,
 * and constructing new backend eventbus implementations
 */
pub mod client;
pub mod msg;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Receiver, Sender};

/**
 * A channel is named and typed with the type of messages it should be carrying
 */
struct Channel {
    name: String,
    send: Sender<Message>,
    recv: Receiver<Message>,
}

struct Message {}

impl Channel {
    fn send(&self, msg: Message) {}
    fn recv(&self, msg: Message) {}
}

struct Bus {
    /**
     * Channels are named and can implement a number of different types. This should
     * allow the Bus to handle different channels with different message payloads
     * while still taking advantage of compile-time checks
     */
    channels: HashMap<String, Channel>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
