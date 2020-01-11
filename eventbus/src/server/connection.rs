/**
 * The client module contains all the logic for handling the client connections to the eventbus.
 *
 * It does _not_ contain a client implementation to the eventbus however
 */
use actix::*;
use actix_web_actors::ws;
use chrono::prelude::Utc;
use log::{error, info, trace};
use serde_json;

use std::sync::Arc;

use crate::*;
use otto_eventbus::*;

/*
 * Define the Websocket Actor needed for Actix
 *
 * This actor will contain state for each connection, and can also have some helper functions to
 * keep track of this connection and its configuration
 */
pub struct WSClient {
    events: Addr<eventbus::EventBus>,
}

/**
 * The WSClient is an actor responsible for the stateful behavior of each websocket
 * connected client
 */
impl WSClient {
    /**
     * Construct the WSClient with the given EventBus actor to communicate with
     */
    pub fn new(eb: Addr<eventbus::EventBus>) -> Self {
        Self { events: eb }
    }

    /**
     * handle_connect() takes care of the Input::Connect message and will ensure
     * that the client is connected to the "all" channel as well as its inbox"
     */
    fn handle_connect(&self, client_name: String, ctx: &mut <WSClient as Actor>::Context) {
        self.events.do_send(eventbus::Subscribe {
            to: "all".to_owned(),
            addr: ctx.address(),
        });

        /*
         * Both of these message sends are creating their own String to represent
         * the inbox channel.
         *
         * This SHOULD be fixed, but will require some more thinking about how
         * the lifetimes of Channel objects should work
         */

        self.events.do_send(eventbus::CreateChannel {
            channel: eventbus::Channel {
                name: format!("inbox.{}", client_name),
                stateful: true,
            },
        });

        self.events.do_send(eventbus::Subscribe {
            to: format!("inbox.{}", client_name),
            addr: ctx.address(),
        });
    }

    /**
     * handle_text handles _all_ incoming messages from the websocket connection,
     * it is responsible for translating those JSON messages into something the
     * eventbus can pass around internally
     */
    fn handle_text(&self, text: String, ctx: &mut <WSClient as Actor>::Context) {
        let command = serde_json::from_str::<InputMessage>(&text);

        match command {
            Ok(c) => {
                // Since we have a Command, what kind?
                match c {
                    InputMessage { msg, meta } => {
                        match msg {
                            Input::Connect { name } => {
                                info!("Received connect for client named: {}", name);
                                self.handle_connect(name, ctx)
                            },

                            Input::Publish { payload } => {
                                info!("received publish: {:?}", payload);
                                self.events.do_send(eventbus::Event {
                                    e: Arc::new(Output::Message { payload: payload }),
                                    channel: Arc::new(meta.channel),
                                });
                            }
                            Input::Subscribe { client } => {
                                info!("Subscribing {} to {}", client, meta.channel);
                                // Sent it along to the bus
                                // TODO: This should not use do_send which ignores errors
                                self.events.do_send(eventbus::Subscribe {
                                    to: meta.channel,
                                    addr: ctx.address(),
                                });
                            }
                            _ => (),
                        };
                    }
                };
            }
            Err(e) => {
                error!("Error parsing message from client: {:?}", e);
            }
        }
    }
}

/**
 * Handle eventbus Output messages by serializing them over to the websocket
 */
impl Handler<eventbus::Event> for WSClient {
    type Result = ();

    /**
     * The `handle` function will be invoked when the WSClient actor receives a message which is
     * intended to be sent to the client via a WebSocket.
     *
     * The handler will serialize the Output, and add additional metadata for the client
     */
    fn handle(&mut self, event: eventbus::Event, ctx: &mut Self::Context) {
        let meta = Meta {
            channel: event.channel.to_string(),
            ts: Utc::now(),
        };
        let out = OutputMessage { msg: event.e, meta };
        // TODO: error
        ctx.text(serde_json::to_string(&out).unwrap());
    }
}

/**
 * Implement the Actor trait, which ensures we can plug into the actix-web-actors tooling for
 * handling websocket connections
 */
impl Actor for WSClient {
    type Context = ws::WebsocketContext<Self>;
}

/**
 * Handler for the ws::Message message for the WSClient actor
 *
 * This handler will be called for every message from a websocket client
 */
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSClient {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };
        trace!("WebSocket message received: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => self.handle_text(text, ctx),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
