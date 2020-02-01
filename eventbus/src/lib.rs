#![allow(unused_imports)]
#![deny(unsafe_code)]
/**
 * The Otto Eventbus module contains public interfaces for sending messages, constructing clients,
 * and constructing new backend eventbus implementations
 */
pub mod client;
pub mod msg;

use log::*;
use tokio::sync::broadcast::{channel, Receiver, SendError, Sender};

use std::collections::HashMap;
use std::sync::Arc;

/**
 * The maximum number of items in transit for each channel
 */
const MAX_CHANNEL_QUEUE: usize = 16;
pub static CHANNEL_ALL: &str = "all";

#[derive(Debug, PartialEq)]
pub struct Event {
    pub m: Arc<msg::Output>,
}

impl Default for Event {
    fn default() -> Event {
        let md = msg::Output::default();
        Event { m: Arc::new(md) }
    }
}

pub type SendableEvent = Arc<Event>;

/**
 * A channel is named and typed with the type of messages it should be carrying
 */
#[derive(Debug)]
struct Channel {
    name: String,
    stateful: bool,
    sender: Sender<Arc<Event>>,
    receiver: Receiver<Arc<Event>>,
}

impl Channel {
    fn send(&self, ev: SendableEvent) -> Result<usize, SendError<SendableEvent>> {
        self.sender.send(ev)
    }

    pub fn new(name: String, stateful: bool) -> Self {
        let (sender, receiver) = tokio::sync::broadcast::channel(MAX_CHANNEL_QUEUE);

        Channel {
            name,
            stateful,
            sender,
            receiver,
        }
    }

    pub fn stateful(name: String) -> Channel {
        Channel::new(name, true)
    }

    pub fn stateless(name: String) -> Channel {
        Channel::new(name, false)
    }

    pub fn is_stateful(&self) -> bool {
        self.stateful == true
    }
}

#[derive(Debug)]
pub struct Bus {
    /**
     * Channels are named and can implement a number of different types. This should
     * allow the Bus to handle different channels with different message payloads
     * while still taking advantage of compile-time checks
     */
    channels: HashMap<String, Channel>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            channels: HashMap::new(),
        }
    }

    /**
     * Configure the bus with a number of stateless channels
     *
     * Stateless channels are not intended to be persisted by the eventbus
     */
    pub fn stateless(&mut self, channels: Vec<String>) -> &mut Bus {
        for channel in channels.iter() {
            self.channels.insert(
                channel.to_string(),
                Channel::new(channel.to_string(), false),
            );
        }
        self
    }

    /**
     * Configure the bus with a number of stateful channels
     */
    pub fn stateful(&mut self, channels: Vec<String>) -> &mut Bus {
        for channel in channels.iter() {
            self.channels
                .insert(channel.to_string(), Channel::new(channel.to_string(), true));
        }
        self
    }

    /**
     * Determine whether the named channel is configured in thebus
     */
    pub fn has_channel(&self, channel: &str) -> bool {
        self.channels.contains_key(channel)
    }

    /**
     * Send an event to the named channel
     */
    pub fn send(
        &self,
        channel: &String,
        ev: SendableEvent,
    ) -> Result<usize, SendError<SendableEvent>> {
        if let Some(c) = self.channels.get(channel) {
            c.send(ev)
        } else {
            Err(SendError(ev))
        }
    }

    /**
     * Create a new receiver for the named channel
     */
    pub fn receiver_for(&self, channel: &str) -> Result<Receiver<SendableEvent>, &str> {
        debug!("receiver_for({})", channel);
        if let Some(c) = self.channels.get(channel) {
            Ok(c.sender.subscribe())
        } else {
            error!("Failed to get channel");
            Err("Fail")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bus_send() {
        let mut b = Bus::new();
        let ch = "test".to_string();
        b.stateless(vec!["test".to_string()]);

        if let Ok(mut rx) = b.receiver_for(&ch) {
            let e = Event::default();
            let p = Arc::new(e);
            if let Ok(value) = b.send(&ch, p.clone()) {
                let value = rx.try_recv().unwrap();
                assert_eq!(p, value);
            } else {
                assert!(false);
            }
        } else {
            /*
             * This branch should never execute unless the test should fail
             */
            assert!(false);
        }
    }

    #[test]
    fn test_bus_stateless() {
        let mut b = Bus::new();
        b.stateless(vec!["test".to_string()]);
        assert!(b.has_channel("test"));
    }

    #[test]
    fn test_bus_stateful() {
        let mut b = Bus::new();
        b.stateful(vec!["test".to_string()]);
        assert!(b.has_channel("test"));
    }

    #[test]
    fn test_channel_ctor() {
        let c = Channel::new("test".to_string(), false);
    }

    #[test]
    fn test_channel_stateful() {
        let c = Channel::stateful("test".to_string());
        assert!(c.is_stateful());
    }

    #[test]
    fn test_channel_stateless() {
        let c = Channel::stateless("test".to_string());
        assert!(!c.is_stateful());
    }
}
