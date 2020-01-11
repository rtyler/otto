/**
 * The bus module contains the actual eventbus actor
 */
use actix::*;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use log::{error, info};

use crate::*;

/*
 * NOTE: I would like for the bus module not to know anything at all about the clients.
 *
 * At the moment I believe that would require a bit more type and generics surgery
 * than I am currently willing to expend on the problem
 */
type ClientId = Addr<connection::WSClient>;

/**
 * The Channel struct is used as an internal representation of each channel that
 * the eventbus knows about.
 *
 * Channels may be either stateless or stateful, with the ladder implying persistence
 * guarantees, depending on the eventbus' backing implementation.
 */
#[derive(Clone, Debug, Eq)]
pub struct Channel {
    pub name: String,
    pub stateful: bool,
}

/**
 * The CreateChannel message is only meant to be used by internal components of
 * the eventbus.
 *
 * It is used primarily for creating new channels on-demand, such as those needed
 * for new client inboxes
 */
#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateChannel {
    pub channel: Channel,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe {
    pub to: String,
    pub addr: ClientId,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Event {
    pub e: Arc<crate::Output>,
    pub channel: Arc<String>,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Unsubscribe {
    pub addr: ClientId,
}

/**
 * Implementation of the Hash trait for Channel ensures that it can be placed in a HashSet
 */
impl Hash for Channel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/**
 * Implementation of PartialEq trait for Channel ensures that it can be placed in a HashSet
 */
impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

/**
 * The EventBus is the main actor inside of this application and acts as the coordination object
 * for sending messages around to the different clients that should receive them
 */
pub struct EventBus {
    channels: HashMap<Channel, HashSet<ClientId>>,
}

/**
 *
 * The Actor trait for the Eventbus allows it to act as an actor in the actix system
 */
impl Actor for EventBus {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Starting Eventbus with channels {:?}", self.channels.keys());
    }
}

impl EventBus {
    /**
     * Configure the EventBus instance with channels
     */
    pub fn with_channels(stateless: Vec<String>, stateful: Vec<String>) -> EventBus {
        let mut channels = HashMap::new();

        for name in stateless {
            channels.insert(
                Channel {
                    name,
                    stateful: false,
                },
                HashSet::new(),
            );
        }
        for name in stateful {
            channels.insert(
                Channel {
                    name,
                    stateful: true,
                },
                HashSet::new(),
            );
        }
        EventBus { channels: channels }
    }
}

impl Handler<CreateChannel> for EventBus {
    type Result = ();

    fn handle(&mut self, create: CreateChannel, _: &mut Context<Self>) {
        self.channels.insert(create.channel, HashSet::new());
    }
}

impl Handler<Subscribe> for EventBus {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Context<Self>) {
        info!("Subscribing client to {}", msg.to);
        // The stateful field doesn't matter here because the hashing on the
        // HashMap only applies to the name of the channel
        let ch = Channel {
            name: msg.to,
            stateful: false,
        };

        if self.channels.contains_key(&ch) {
            match self.channels.get_mut(&ch) {
                Some(set) => {
                    set.insert(msg.addr);
                }
                None => {
                    error!("Failed to access the set");
                }
            }
        } else {
            error!("No channel named `{}` configured", ch.name);
        }
    }
}

impl Handler<Event> for EventBus {
    type Result = ();

    fn handle(&mut self, ev: Event, _: &mut Context<Self>) {
        let ch = Channel {
            name: ev.channel.to_string(),
            stateful: false,
        };

        if self.channels.contains_key(&ch) {
            if let Some(clients) = self.channels.get(&ch) {
                /*
                 * NOTE: In the future this might need to be a more parallel iteration or something
                 * which will handle numerous simultaneous client connections.
                 *
                 * For now it is safe to assume we're going to have relatively few clients on the
                 * eventbus
                 */
                for client in clients {
                    /*
                     * TODO: cloning this is probably not necessary but I do not currently
                     * understand how to pass references to Actors in actix.
                     */
                    client.do_send(ev.clone());
                }
            }
        } else {
            error!("Received an event for a non-existent channel: {}", ch.name);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_with_channels_empty() {
        let _bus = EventBus::with_channels(vec![], vec![]);
    }

    #[test]
    fn test_with_channels_stateless() {
        let _bus = EventBus::with_channels(vec![String::from("test")], vec![]);
    }
}
