/**
 * The bus module contains the actual eventbus actor
 */
use actix::*;
use std::collections::{HashMap, HashSet};

use log::{error, info};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Msg(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe {
    pub to: String,
    pub addr: Addr<crate::client::WSClient>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Event {
    pub m: String,
    pub channel: String,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Unsubscribe {
    pub from: String,
}

type ClientId = Addr<crate::client::WSClient>;

/**
 * The EventBus is the main actor inside of this application and acts as the coordination object
 * for sending messages around to the different clients that should receive them
 */
pub struct EventBus {
    /**
     * channels is the map of string to the clients currently connected to that channel by
     * clientId.
     *
     * It is assumed that the EventBus can handle holding onto a client identifier for each and
     * every client that connects (64 bits should be enough for that)
     */
    channels: HashMap<String, HashSet<ClientId>>,
}

impl Actor for EventBus {
    type Context = Context<Self>;
}

impl Default for EventBus {
    fn default() -> EventBus {
        let mut channels = HashMap::new();
        channels.insert("all".to_owned(), HashSet::new());

        EventBus { channels: channels }
    }
}

impl Handler<Subscribe> for EventBus {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Context<Self>) {
        if self.channels.contains_key(&msg.to) {
            info!("Client subscribing to {}", msg.to);
            match self.channels.get_mut(&msg.to) {
                Some(set) => {
                    set.insert(msg.addr);
                }
                None => {
                    error!("Failed to access the set");
                }
            }
        } else {
            error!("No channel named `{}` configured", msg.to);
        }
    }
}

impl Handler<Event> for EventBus {
    type Result = ();

    fn handle(&mut self, msg: Event, _: &mut Context<Self>) {
        if self.channels.contains_key(&msg.channel) {
            info!("Bus message for {}", msg.channel);
            info!("  -> {}", msg.m);

            if let Some(clients) = self.channels.get(&msg.channel) {
                for client in clients {
                    client.do_send(Msg(msg.m.to_owned()));
                }
            }
        } else {
            error!(
                "Received an event for a non-existent channel: {}",
                msg.channel
            );
        }
    }
}
