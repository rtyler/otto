/**!
 * This module is the eventbus client
 */
use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket, Client,
};
use bytes::Bytes;
use futures::stream::{SplitSink, StreamExt};
use log::{error, info};

use std::time::Duration;

use crate::*;

/**
 * An EventBusClient is capable of connecting to, reading messages from, and sending messages to
 * the eventbus.
 */
pub struct EventBusClient {
    /**
     * The sink is a writable object which can send messages back to the EventBus
     */
    sink: SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>,
    /**
     * String identifier for the client
     *
     * This should be persisted between invocations of the process if the client
     * should have any semblence of persistence with its messages
     */
    id: &'static str,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Disconnect;

/**
 * Implementation of the Debug trait so that the EventBusClient can be included
 * in logging statements and print each instance's `id`
 */
impl std::fmt::Debug for EventBusClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventBusClient<id:{}>", self.id)
    }
}

/**
 * connect will create begin the connection process and start the EventBusClient
 * actor.
 *
 * The caller will need to await in order to get the EventBusClient
 */
pub async fn connect(ws: &'static str, id: &'static str) -> Addr<EventBusClient> {
    let mut backoff = 1;

    loop {
        let r = Client::new().ws(ws).connect().await;

        match r {
            Ok((response, framed)) => {
                let (sink, stream) = framed.split();

                return EventBusClient::create(|ctx| {
                    EventBusClient::add_stream(stream, ctx);
                    EventBusClient {
                        sink: SinkWrite::new(sink, ctx),
                        id: id,
                    }
                });
            }
            Err(e) => {
                error!("Failed establish WebSocket: {}", e);
                backoff = backoff + backoff;
                std::thread::sleep(Duration::from_secs(backoff));
            }
        }
    }
}

impl EventBusClient {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            act.sink
                .write(Message::Ping(Bytes::from_static(b"")))
                .unwrap();
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

impl Actor for EventBusClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        let input = InputMessage {
            meta: Meta::default(),
            msg: Input::Connect {
                name: self.id.to_string(),
            },
        };
        self.sink
            .write(Message::Text(serde_json::to_string(&input).unwrap()))
            .unwrap();
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        info!("Disconnected");
    }
}

impl Handler<Disconnect> for EventBusClient {
    type Result = ();

    fn handle(&mut self, _: Disconnect, ctx: &mut Context<Self>) {
        ctx.stop()
    }
}

impl Handler<InputMessage> for EventBusClient {
    type Result = ();

    fn handle(&mut self, input: InputMessage, _ctx: &mut Context<Self>) {
        self.sink
            .write(Message::Text(serde_json::to_string(&input).unwrap()))
            .unwrap();
    }
}

impl Handler<OutputMessage> for EventBusClient {
    type Result = ();

    fn handle(&mut self, _output: OutputMessage, _ctx: &mut Context<Self>) {
        /*
         * For heartbeats we really don't need to do anything
         */
    }
}

/// Handle server websocket messages
impl StreamHandler<Result<Frame, WsProtocolError>> for EventBusClient {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, ctx: &mut Context<Self>) {
        if let Ok(Frame::Text(txt)) = msg {
            /*
             * We have received _some_ message from the eventbus, let's try to
             * decode it!
             */
            let msg = serde_json::from_slice::<OutputMessage>(&txt);
            match msg {
                Ok(output) => {
                    /*
                     * Dispatch the message basically back to ourself for easy
                     * handling
                     */
                    ctx.address().do_send(output);
                }
                Err(e) => {
                    error!("Received invalid message: {}", e);
                }
            }
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        info!("Server disconnected");
        ctx.stop()
    }
}

impl actix::io::WriteHandler<WsProtocolError> for EventBusClient {}

#[cfg(test)]
mod test {}
