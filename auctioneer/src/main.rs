/**!
 * The auctioneer main model
 */
extern crate actix;
extern crate actix_http;
extern crate actix_web;
extern crate awc;
extern crate chrono;
extern crate pretty_env_logger;

use actix::*;
use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket, Client,
};
use actix::io::SinkWrite;
use actix_web::{middleware, web};
use actix_web::{App, HttpResponse, HttpServer};
use bytes::Bytes;
use chrono::Utc;
use futures::stream::{SplitSink, StreamExt};
use log::{info, error};

use std::time::Duration;

use otto_eventbus::*;

/**
 * The index handler for the root of the Auctioneer web interface
 */
async fn route_index() -> HttpResponse {
    HttpResponse::Ok().body("Auctioneer")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    /*
     * Creating a awc::Client to handle the first part of our WebSocket client bootstrap
     */
    let (response, framed) = Client::new()
        .ws("http://127.0.0.1:8000/ws/")
        .connect()
        .await
        .map_err(|e| {
            error!("Error: {}", e);
        })
        .unwrap();

    info!("{:?}", response);
    let (sink, stream) = framed.split();
    let addr = EventBusClient::create(|ctx| {
        EventBusClient::add_stream(stream, ctx);
        EventBusClient(SinkWrite::new(sink, ctx))
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(route_index))
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}

struct EventBusClient(SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>);

#[derive(Message)]
#[rtype(result = "()")]
struct ClientCommand(String);

impl Actor for EventBusClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        let input = InputMessage {
            meta: Meta {
                channel: "".to_string(),
                ts: Utc::now(),
            },
            msg: Input::Connect {
                name: "auctioneer".to_string(),
            },
        };
        self.0.write(Message::Text(serde_json::to_string(&input).unwrap())).unwrap();
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        info!("Disconnected");
    }
}

impl EventBusClient {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            act.0.write(Message::Ping(Bytes::from_static(b""))).unwrap();
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

/// Handle stdin commands
impl Handler<ClientCommand> for EventBusClient {
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        self.0.write(Message::Text(msg.0)).unwrap();
    }
}

/// Handle server websocket messages
impl StreamHandler<Result<Frame, WsProtocolError>> for EventBusClient {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        if let Ok(Frame::Text(txt)) = msg {
            /*
             * We have received _some_ message from the eventbus, let's try to
             * decode it!
             */
            let msg = serde_json::from_slice::<OutputMessage>(&txt);
            match msg {
                Ok(msg) => {
                    info!("Received valid message: {:?}", msg);
                },
                Err(e) => {
                    error!("Received invalid message: {}", e);
                },
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
mod test {
    use super::*;
    use actix_web::{test, web, App};

    /**
     * This test just ensures that the server can come online properly and render its index handler
     * properly.
     */
    #[actix_rt::test]
    async fn test_basic_http() {
        let srv = test::start(move || {
            App::new()
                .route("/", web::get().to(route_index))
        });

        let req = srv.get("/");
        let response = req.send().await.unwrap();
        assert!(response.status().is_success());
    }
}
