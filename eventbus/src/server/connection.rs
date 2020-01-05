/**
 * The client module contains all the logic for handling the client connections to the eventbus.
 *
 * It does _not_ contain a client implementation to the eventbus however
 */
use actix::*;
use actix_web_actors::ws;
use chrono::prelude::Utc;
use log::{error, info};
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

impl WSClient {
    pub fn new(eb: Addr<eventbus::EventBus>) -> Self {
        Self { events: eb }
    }

    fn handle_text(&self, text: String, ctx: &<WSClient as Actor>::Context) {
        let command = serde_json::from_str::<Input>(&text);

        match command {
            Ok(c) => {
                // Since we have a Command, what kind?
                match c {
                    Input::Subscribe { client, channel } => {
                        info!("Subscribing {} to {}", client, channel);
                        // Sent it along to the bus
                        // TODO: This should not use do_send which ignores errors
                        self.events.do_send(eventbus::Subscribe {
                            to: channel,
                            addr: ctx.address(),
                        });
                    }
                    _ => (),
                }
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
        let out = OutputMessage {
            msg: event.e,
            meta,
        };
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

    fn started(&mut self, ctx: &mut Self::Context) {
        let sub = eventbus::Subscribe {
            to: "all".to_owned(),
            addr: ctx.address(),
        };
        self.events
            .send(sub)
            .into_actor(self)
            .then(|result, _actor, ctx| {
                match result {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSClient {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };
        info!("WebSocket received: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => self.handle_text(text, ctx),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}
