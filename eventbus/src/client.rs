/**
 * The client module contains all the logic for handling the client connections to the eventbus.
 *
 * It does _not_ contain a client implementation to the eventbus however
 */
use actix::*;
use actix_web_actors::ws;
use log::{error, info};
use serde_json;

use std::sync::Arc;

/*
 * Define the Websocket Actor needed for Actix
 *
 * This actor will contain state for each connection, and can also have some helper functions to
 * keep track of this connection and its configuration
 */
pub struct WSClient {
    events: Addr<crate::bus::EventBus>,
}

impl WSClient {
    pub fn new(eb: Addr<crate::bus::EventBus>) -> Self {
        Self { events: eb }
    }

    fn handle_text(&self, text: String) {
        let command = serde_json::from_str::<crate::Command>(&text);

        match command {
            Ok(c) => {
                // Since we have a Command, what kind?
                match c {
                    crate::Command::Subscribe { client, channel } => {
                        info!("Subscribing {} to {}", client, channel);
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
 * Handle Basic eventbus messages by serializing them over to the websocket
 */
impl Handler<Arc<crate::Command>> for WSClient {
    type Result = ();

    fn handle(&mut self, msg: Arc<crate::Command>, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

/**
 * Implement the Actor trait, which ensures we can plug into the actix-web-actors tooling for
 * handling websocket connections
 */
impl Actor for WSClient {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let sub = crate::bus::Subscribe {
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
            ws::Message::Text(text) => self.handle_text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}
