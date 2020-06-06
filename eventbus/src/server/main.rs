#![allow(unused_imports)]
#![allow(dead_code)]

/**
 * The main module for otto-eventbus simply sets up the responders and the main
 * server loop that the eventbus uses.
 */
extern crate async_std;
extern crate config;
extern crate futures;
extern crate http;
extern crate log;
extern crate mime;
extern crate pretty_env_logger;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate serde_json;
extern crate tide;

use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::*;
use chrono::Local;
use config::Config;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    stream::TryStreamExt,
    stream::select,
    SinkExt, StreamExt,
};

use handlebars::Handlebars;
use http::status::StatusCode;
use log::{debug, error, info, trace};
use serde::Serialize;
use tide::{Request, Response, Server};

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

struct State {
    hb: Arc<Handlebars>,
    conf: Arc<Config>,
}

async fn index(ctx: Request<State>) -> Response {
    let conf = &ctx.state().conf;

    let bind = conf.get_str("ws.bind").expect("Could not locate ws.bind");
    let port = conf.get_int("ws.port").expect("Could not locate ws.port");

    let res = Response::new(200);
    let data = json!({
        "version" : option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        "ws" : format!("{}:{}", bind, port),
    });

    if let Ok(view) = ctx.state().hb.render("index.html", &data) {
        return res.body_string(view).set_mime(mime::TEXT_HTML);
    } else {
        error!("Failed to render");
        return res
            .set_status(StatusCode::INTERNAL_SERVER_ERROR)
            .body_string("Internal server error".to_string());
    }
}

/**
 * Load the hierarchy of settings and return the configuration object
 */
fn load_configuration() -> Config {
    let embedded = Static::get("eventbus.yml").expect("Failed to load built-in/static settings");
    let defaults = std::str::from_utf8(embedded.as_ref()).unwrap();
    /*
     * Load our settings in the priority order of:
     *
     *   - built-in defaults
     *   - yaml file
     *   - environment variables
     *
     * Each layer overriding properties from the last
     */
    let mut conf = Config::default();
    conf.merge(config::File::from_str(defaults, config::FileFormat::Yaml))
        .unwrap()
        .merge(config::File::with_name("eventbus").required(false))
        .unwrap()
        .merge(config::Environment::with_prefix("OTTO_EB"))
        .unwrap();

    let motd: String = conf
        .get("motd")
        .expect("Configuration had no `motd` setting");

    debug!("configured motd: {}", motd);
    return conf;
}

/**
 * Load the Handlebars templates needed for presenting the web UI
 */
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

struct Connection {
    stream: WebSocketStream<TcpStream>,
    inbox: UnboundedReceiver<String>,
}

async fn handle_ws(mut c: Connection) -> Result<(), std::io::Error> {
    while let Some(item) = select(c.stream, c.inbox).await {
        println!("Received: {:?}", item);
    }
    //while let Some(msg) = c.stream.next().await {

    //    if let Err(e) = c.stream.send(Message::text("{\"m\": \"Hello sailor\"}".to_string())).await {
    //        error!("Failed to send a message to a connection: {}", e);
    //    }
    //}

    //Ok(())
}

/**
 * Create the WebSocket listener for accepting commands and pushing events
 */
async fn serve_ws(conf: Arc<config::Config>) -> Result<(), std::io::Error> {
    let bind: String = conf
        .get("ws.bind")
        .expect("Invalid `ws.bind` configuration, must be a string (e.g. '127.0.0.1')");
    let port: u64 = conf
        .get("ws.port")
        .expect("Invalid `ws.port` configuration, must be an integer");

    let listener = TcpListener::bind(format!("{}:{}", bind, port))
        .await
        .expect(format!("Failed to bind to {}", port).as_str());

    info!("Listening for WebSocket connections on {}:{}", bind, port);

    let mut txs: Vec<UnboundedSender<String>> = vec![];

    while let Ok((stream, _)) = listener.accept().await {
        let ws = accept_async(stream)
            .await
            .expect("Error during the WebSocket handshake occurred");

        let (tx, inbox) = unbounded();
        txs.push(tx);

        let conn = Connection {
            stream: ws,
            inbox,
        };

        let _handle = task::spawn(handle_ws(conn));
    }

    Ok(())
}

/**
 * Create the tide HTTP listener for handling conventional web requests
 *
 * This is mostly used for serving up the web UI for the eventbus
 */
async fn serve_http(conf: Arc<config::Config>, state: State) -> Result<(), std::io::Error> {
    let bind: String = conf
        .get("http.bind")
        .expect("Invalid `http.bind` configuration, must be a string (e.g. '127.0.0.1')");
    let port: u64 = conf
        .get("http.port")
        .expect("Invalid `http.port` configuration, must be an integer");

    let mut app = Server::with_state(state);

    app.at("/").get(index);

    info!("Listening for HTTP connections on {}:{}", bind, port);
    app.listen(format!("{}:{}", bind, port)).await?;

    info!("HTTP listener exiting..");
    Ok(())
}

fn main() {
    pretty_env_logger::init();
    info!("Initializing the Otto Eventbus");
    let config = load_configuration();
    let conf = Arc::new(config);

    let mut hb = Handlebars::new();
    load_templates(&mut hb);
    let hb = Arc::new(hb);
    let state = State {
        hb: hb,
        conf: conf.clone(),
    };

    let _seconds: u64 = conf
        .get("heartbeat")
        .expect("Invalid `heartbeat` configuration, must be an integer");

    task::spawn(serve_ws(conf.clone()));
    task::block_on(serve_http(conf.clone(), state)).expect("Failed to run the main runloop");

    //thread::spawn(move || loop {
    //    let ts = Local::now();
    //    let pulse = format!("heartbeat {}", ts);
    //    info!("sending pulse: {}", pulse);
    //    let hb = msg::Output::Heartbeat {};
    //    let e = Event { m: Arc::new(hb) };
    //    hb_bus.send(&"all".to_string(), Arc::new(e));
    //    thread::sleep(Duration::from_secs(seconds));
    //});
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_conf() {
        let c = load_configuration();
        let motd: String = c.get("motd").unwrap();
        let pulse: u64 = c.get("heartbeat").unwrap();
        assert!(motd.len() > 0);
        assert_eq!(pulse, 60);
    }
}
