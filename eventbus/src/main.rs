/**
 * The main module for otto-eventbus simply sets up the responders and the main
 * server loop that the eventbus uses.
 */
extern crate actix;
extern crate actix_web;
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate serde_json;

use actix::{Actor, Addr};
use actix_web::{middleware, web};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use chrono::Local;
use handlebars::Handlebars;
use log::trace;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub mod bus;
mod client;
mod msg;

#[derive(RustEmbed)]
#[folder = "eventbus/templates"]
// Templates is a rust-embed struct which will contain all the files embedded from the
// eventbus/templates/ directory
struct Templates;

#[derive(Clone)]
struct AppState {
    bus: Addr<bus::EventBus>,
    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    hb: Arc<Handlebars>,
}

/**
 * index serves up the homepage which is not really functional, but at least shows lost users
 * something
 */
async fn index(state: web::Data<AppState>) -> HttpResponse {
    let data = json!({
        "version" : option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
    });

    let template = Templates::get("index.html").unwrap();
    let body = state
        .hb
        .render_template(std::str::from_utf8(template.as_ref()).unwrap(), &data)
        .expect("Failed to render the index.html template!");
    HttpResponse::Ok().body(body)
}

/**
 * ws_index is the handler for all websocket connections, all it is responsible for doing is
 * handling the inbound connection and creating a new WSClient for the connection
 */
async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let actor = client::WSClient::new(state.bus.clone());
    let res = ws::start(actor, &r, stream);
    trace!("{:?}", res.as_ref().unwrap());
    res
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    /*
     * The EventBus needs our Channel `directory` in order to receive messages and dispatch them
     * appropiately
     */
    let events = bus::EventBus::default().start();
    let bus = events.clone();

    thread::spawn(move || loop {
        let pulse = format!("heartbeat {}", Local::now());
        trace!("sending pulse: {}", pulse);
        let event = crate::bus::Event {
            m: pulse,
            channel: "all".to_string(),
        };
        bus.do_send(event);
        thread::sleep(Duration::from_millis(30000));
    });

    let state = AppState {
        bus: events,
        hb: Arc::new(Handlebars::new()),
    };
    let wd = web::Data::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(wd.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/ws/", web::get().to(ws_index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
