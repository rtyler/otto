/**
 * The bus module contains the actual eventbus actor
 */
use actix::*;
use std::collections::{HashMap, HashSet};
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
    pub e: Arc<crate::Basic>,
    pub channel: String,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Unsubscribe {
    pub from: String,
}

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

    fn handle(&mut self, ev: Event, _: &mut Context<Self>) {
        if self.channels.contains_key(&ev.channel) {
            info!("Bus message for {}", ev.channel);

            if let Some(clients) = self.channels.get(&ev.channel) {
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
            error!(
                "Received an event for a non-existent channel: {}",
                ev.channel
            );
        }
    }
}
