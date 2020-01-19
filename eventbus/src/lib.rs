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
struct ChannelV<T> {
    name: String,
    send: Sender<T>,
    recv: Receiver<T>,
}

struct StringChannel {
    name: String,
    send: Sender<String>,
    recv: Receiver<String>,
}
struct SM(String);
impl Message for SM {}

impl Channel<SM> for StringChannel {
    fn send(&self, msg: SM) {}
    fn recv(&self, msg: SM) {}
}

trait Message {}

trait Channel<T> {
    fn send(&self, msg: impl Message);
    fn recv(&self, msg: impl Message);
}

struct Bus {
    /**
     * Channels are named and can implement a number of different types. This should
     * allow the Bus to handle different channels with different message payloads
     * while still taking advantage of compile-time checks
     */
    channels: HashMap<String, Box<Channel<Message>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
