#![allow(unused_imports)]
#![allow(dead_code)]

/**
 * The main module for otto-eventbus simply sets up the responders and the main
 * server loop that the eventbus uses.
 */
extern crate config;
extern crate futures;
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate serde_json;

use chrono::Local;
use futures::future;
use futures::stream::{SplitSink, SplitStream};
use futures::{FutureExt, SinkExt, StreamExt};
use handlebars::Handlebars;
use log::{debug, error, info, trace};
use serde::Serialize;
use tokio::sync::broadcast::Receiver;
use warp::reject::Rejection;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use otto_eventbus::*;

/**
 * Templates is a rust-embed struct which will contain all the files embedded from the
 * eventbus/templates/ directory
 */
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/templates"]
struct Templates;

/**
 * Static is a rust-embed struct which contains everything in the static/
 * folder at compile-time
 */
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/static"]
struct Static;

async fn index(hb: Arc<Handlebars>) -> Result<impl warp::Reply, Rejection> {
    let data = json!({
        "version" : option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
    });

    if let Ok(view) = hb.render("index.html", &data) {
        Ok(warp::reply::html(view))
    } else {
        error!("Failed to render");
        Ok(warp::reply::html("Fail".to_string()))
    }
}

/**
 * Return a filter which will proppogate a Handlebars Arc
 */
fn with_render(
    hb: Arc<Handlebars>,
) -> impl Filter<Extract = (Arc<Handlebars>,), Error = Infallible> + Clone {
    warp::any().map(move || hb.clone())
}

/**
 * Load the hierarchy of settings and return the configuration object
 */
fn load_settings() -> config::Config {
    let embedded_settings =
        Static::get("eventbus.yml").expect("Failed to load built-in/static settings");
    let defaults = std::str::from_utf8(embedded_settings.as_ref()).unwrap();
    /*
     * Load our settings in the priority order of:
     *
     *   - built-in defaults
     *   - yaml file
     *   - environment variables
     *
     * Each layer overriding properties from the last
     */
    let mut settings = config::Config::default();
    settings
        .merge(config::File::from_str(defaults, config::FileFormat::Yaml))
        .unwrap()
        .merge(config::File::with_name("eventbus").required(false))
        .unwrap()
        .merge(config::Environment::with_prefix("OTTO_EB"))
        .unwrap();

    let motd: String = settings
        .get("motd")
        .expect("Configuration had no `motd` setting");

    debug!("configured motd: {}", motd);
    return settings;
}

fn load_templates(hb: &mut Handlebars) {
    for t in Templates::iter() {
        if !t.ends_with(".html") {
            continue;
        }

        let template = Templates::get(&t)
            .expect("Somehow we iterated Templates but didn't get one? How is this possible!");
        let buf = std::str::from_utf8(template.as_ref())
            .expect(format!("Unable to convert {} to a string buffer", &t).as_str());
        hb.register_template_string(&t, buf)
            .expect(format!("Failed to register {} as a Handlebars template", &t).as_str());

        info!("Registered handlebars template: {}", &t);
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let settings = load_settings();

    let mut hb = Handlebars::new();
    load_templates(&mut hb);
    let hb = Arc::new(hb);

    let mut bus = Bus::new();
    bus.stateless(
        settings
            .get::<Vec<String>>("channels.stateless")
            .expect("Failed to load `channels.stateless`"),
    );
    bus.stateful(
        settings
            .get::<Vec<String>>("channels.stateful")
            .expect("Failed to load `channels.stateful`"),
    );
    let event_bus = Arc::new(bus);
    let hb_bus = event_bus.clone();
    // Create a filter for hte eventbus
    let event_bus = warp::any().map(move || event_bus.clone());

    let seconds = settings
        .get("heartbeat")
        .expect("Invalid `heartbeat` configuration, must be an integer");

    thread::spawn(move || loop {
        let ts = Local::now();
        let pulse = format!("heartbeat {}", ts);
        info!("sending pulse: {}", pulse);
        let hb = msg::Output::Heartbeat {};
        let e = Event { m: Arc::new(hb) };
        hb_bus.send(&"all".to_string(), Arc::new(e));
        thread::sleep(Duration::from_secs(seconds));
    });

    let index = warp::path::end().and(with_render(hb)).and_then(index);

    let ws = warp::path("ws").and(warp::ws()).and(event_bus.clone()).map(
        move |ws: warp::ws::Ws, bus: Arc<Bus>| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(move |websocket| {
                info!("Connection established for {:?}", websocket);
                let (tx, rx) = websocket.split();

                // Clone a reference of the bus for this WebSocket connection
                let bus_conn = bus.clone();
                tokio::task::spawn(async {
                    let mut conn = Connection::new(tx, rx, bus_conn);
                    conn.runloop().await;
                });
                future::ready(())
            })
        },
    );

    let routes = warp::get().and(index.or(ws));
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

type WsOutput = SplitSink<WebSocket, Message>;
type WsInput = SplitStream<WebSocket>;

#[derive(Debug)]
struct Connection {
    tx: WsOutput,
    rx: WsInput,
    bus: Arc<Bus>,
}

impl Connection {
    fn new(tx: WsOutput, rx: WsInput, bus: Arc<Bus>) -> Self {
        Connection {
            tx,
            rx,
            bus,
        }
    }

    async fn runloop(&mut self) {
        let reader = self.rx.by_ref().for_each(|item| {
            info!("Item received: {:?}", item);
            future::ready(())
        });

        let writer = tokio::task::spawn(async {
            /*
             * NOTE: we need to wait for messages to come in on some channel here?
             * busy loop
             */
            debug!("Starting writer task for connection {:?}", self);
            let bus_rx = self.bus.receiver_for("all").unwrap();

            loop {
                match bus_rx.recv().await {
                    Ok(ev) => {
                        info!("Need to dispatch: {:?}", ev);
                    },
                    Err(err) => {
                        error!("Failed to listen to channel: {:?}", err);
                    },
                }
            }
        });

        // TODO handle errors on the join
        future::join(reader, writer).await;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_settings() {
        let c = load_settings();
        let motd: String = c.get("motd").unwrap();
        let pulse: u64 = c.get("heartbeat").unwrap();
        assert!(motd.len() > 0);
        assert_eq!(pulse, 60);
    }
}
