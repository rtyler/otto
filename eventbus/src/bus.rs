/**
 * The bus module contains the actual eventbus actor
 */
use actix::{Actor, Context, Handler, Message, Recipient};
use std::collections::{HashMap, HashSet};

use log::info;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Msg(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe {
    pub to: String,
    pub addr: Recipient<Msg>,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Unsubscribe {
    pub from: String,
}

/**
 * ClientId is just an internal identifier which can be used to map messages to clients.
 *
 * It is currently an unsigned 64-bit integer, but that is an implementation detail which may
 * change in the future
 */
type ClientId = u64;

/**
 * The EventBus is the main actor inside of this application and acts as the coordination object
 * for sending messages around to the different clients that should receive them
 */
pub struct EventBus {
    clients: HashMap<ClientId, u64>,
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

        EventBus {
            clients: HashMap::new(),
            channels: channels,
        }
    }
}

impl Handler<Subscribe> for EventBus {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Context<Self>) {
        info!("Client subscribing to {}", msg.to);
    }
}
