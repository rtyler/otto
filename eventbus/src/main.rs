/**
 * The main module for otto-eventbus simply sets up the responders and the main
 * server loop that the eventbus uses.
 */

extern crate actix;
#[macro_use]
extern crate actix_web;
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate serde_json;

use actix::{Actor, StreamHandler};
use actix_web::{middleware, web};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use handlebars::Handlebars;

#[derive(RustEmbed)]
#[folder = "eventbus/templates"]
// Templates is a rust-embed struct which will contain all the files embedded from the
// eventbus/templates/ directory
struct Templates;

/*
 * Define the Websocket Actor needed for Actix
 */
struct WSActor;

impl Actor for WSActor {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for WSActor {
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

// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "name": "Handlebars"
    });
    let template = Templates::get("index.html").unwrap();
    let body = hb.render_template(std::str::from_utf8(template.as_ref()).unwrap(),
        &data).unwrap();
    HttpResponse::Ok().body(body)
}

/// do websocket handshake and start `MyWebSocket` actor
fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let actor = WSActor { };
    let res = ws::start(actor, &r, stream);
    println!("{:?}", res.as_ref().unwrap());
    res
}

fn main() {
    pretty_env_logger::init();

    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let handlebars = Handlebars::new();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .register_data(handlebars_ref.clone())
            .service(index)
            .route("/ws/", web::get().to(ws_index))
    })
    .bind("127.0.0.1:8000").unwrap()
    .run().unwrap();
}
