/**
 * The bus module contains the actual eventbus actor
 */
use actix::*;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use log::{error, info};

/*
 * NOTE: I would like for the bus module not to know anything at all about the clients.
 *
 * At the moment I believe that would require a bit more type and generics surgery
 * than I am currently willing to expend on the problem
 */
type ClientId = Addr<crate::client::WSClient>;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe {
    pub to: String,
    pub addr: ClientId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Event {
    pub e: Arc<crate::Command>,
    pub channel: String,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Unsubscribe {
    pub addr: ClientId,
}

/**
 * The Channel struct is used as an internal representation of each channel that
 * the eventbus knows about.
 *
 * Channels may be either stateless or stateful, with the ladder implying persistence
 * guarantees, depending on the eventbus' backing implementation.
 */
#[derive(Debug, Eq)]
pub struct Channel {
    name: String,
    stateful: bool,
}

impl Hash for Channel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
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

impl Handler<Subscribe> for EventBus {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Context<Self>) {
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
            name: ev.channel,
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
                    client.do_send(ev.e.clone());
                }
            }
        } else {
            error!("Received an event for a non-existent channel: {}", ch.name);
        }
    }
}
