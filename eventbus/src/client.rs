/**
 * The client module contains all the logic for handling the client connections to the eventbus.
 *
 * It does _not_ contain a client implementation to the eventbus however
 */
use actix::*;
use actix_web_actors::ws;
use log::info;

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
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<crate::bus::Msg> for WSClient {
    type Result = ();

    fn handle(&mut self, msg: crate::bus::Msg, ctx: &mut Self::Context) {
        ctx.text(msg.0);
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
            addr: ctx.address().recipient(),
        };
        self.events
            .send(sub)
            .into_actor(self)
            .then(|result, _actor, ctx| {
                match result {
                    Ok(result) => (),
                    _ => ctx.stop(),
                }
                actix::fut::ok(())
            })
            .wait(ctx);
    }
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for WSClient {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        info!("WebSocket received: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}
