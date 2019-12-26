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

use actix::{Actor, Addr, System};
use actix_web::{middleware, web};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use crossbeam::channel::{unbounded, Receiver, Sender};
use handlebars::Handlebars;
use log::{debug, trace};
use std::collections::HashMap;

use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

pub mod bus;
mod client;
mod msg;

#[derive(RustEmbed)]
#[folder = "eventbus/templates"]
// Templates is a rust-embed struct which will contain all the files embedded from the
// eventbus/templates/ directory
struct Templates;

/**
 * index serves up the homepage which is not really functional, but at least shows lost users
 * something
 */
#[get("/")]
fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "version" : option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
    });

    let template = Templates::get("index.html").unwrap();
    let body = hb
        .render_template(std::str::from_utf8(template.as_ref()).unwrap(), &data)
        .expect("Failed to render the index.html template!");
    HttpResponse::Ok().body(body)
}

/**
 * ws_index is the handler for all websocket connections, all it is responsible for doing is
 * handling the inbound connection and creating a new WSClient for the connection
 */
fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    eb: web::Data<Addr<bus::EventBus>>,
) -> Result<HttpResponse, Error> {
    let actor = client::WSClient::new(eb.get_ref().clone());
    let res = ws::start(actor, &r, stream);
    trace!("{:?}", res.as_ref().unwrap());
    res
}

fn main() {
    pretty_env_logger::init();

    let sys = System::new("ws-example");

    /*
     * The directory should contain the mapping for all channels that are available in the eventbus
     */
    type SendReceiveRefs = (Arc<Sender<String>>, Arc<Receiver<String>>);
    let mut directory: HashMap<String, SendReceiveRefs> = HashMap::new();
    let (tx, rx) = unbounded();
    let send_ref = Arc::new(tx);
    let recv_ref = Arc::new(rx);

    let t = Arc::clone(&send_ref);
    let t1 = Arc::clone(&send_ref);
    // Starting with a common chan
    directory.insert(String::from("all"), (t1, recv_ref));

    let dir_ref = web::Data::new(directory);

    /*
     * The EventBus needs our Channel `directory` in order to receive messages and dispatch them
     * appropiately
     */
    let events = bus::EventBus::default().start();


    thread::spawn(move || loop {
        debug!("pulse");
        t.send(format!("heartbeat {:?}", SystemTime::now()));
        thread::sleep(Duration::from_millis(3000));
    });

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
            .register_data(dir_ref.clone())
            .data(events.clone())
            .service(index)
            .route("/ws/", web::get().to(ws_index))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .start();

    sys.run().unwrap();
}
