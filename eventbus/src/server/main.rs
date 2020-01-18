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
use futures::{FutureExt, StreamExt};
use handlebars::Handlebars;
use log::{debug, error, info, trace};
use serde::Serialize;
use warp::Filter;
use warp::reject::Rejection;

use std::convert::Infallible;
use std::sync::Arc;
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
    }
    else {
        error!("Failed to render");
        Ok(warp::reply::html("Fail".to_string()))
    }
}

/**
 * Return a filter which will proppogate a Handlebars Arc
 */
fn with_render(hb: Arc<Handlebars>) -> impl Filter<Extract = (Arc<Handlebars>,), Error=Infallible> + Clone {
    warp::any().map(move || hb.clone())
}

/**
 * Load the hierarchy of settings and return the configuration object
 */
fn load_settings() -> config::Config {
    let embedded_settings = Static::get("eventbus.yml").expect("Failed to load built-in/static settings");
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
        if ! t.ends_with(".html") {
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
    let mut hb = Handlebars::new();
    load_templates(&mut hb);
    let hb = Arc::new(hb);
    let _settings = load_settings();

    let index = warp::path::end().and(with_render(hb)).and_then(index);
    let ws = warp::path("ws")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        error!("websocket error: {:?}", e);
                    }
                })
            })
        });
    let routes = warp::get().and(index.or(ws));

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    use regex::Regex;

    #[test]
    fn test_load_settings() {
        let c = load_settings();
        let motd: String = c.get("motd").unwrap();
        let pulse: u64 = c.get("heartbeat").unwrap();
        assert!(motd.len() > 0);
        assert_eq!(pulse, 60);
    }

    /**
     * This test just ensures that the server can come online properly and render its index handler
     * properly.
     *
     * It doesn't really test much useful, but does ensure that critical failures in the eventbus
     * can sometimes be prevented
     */
    #[tokio::test]
    async fn test_basic_http() {
        /*
        let events = eventbus::EventBus::with_channels(vec![], vec![]).start();
        let state = AppState {
            bus: events,
            hb: Arc::new(Handlebars::new()),
        };
        let wd = web::Data::new(state);
        let srv = test::start(move || {
            App::new()
                .app_data(wd.clone())
                .route("/", web::get().to(index))
        });

        let req = srv.get("/");
        let mut response = req.send().await.unwrap();
        assert!(response.status().is_success());

        let re = Regex::new(r"(v\d\.\d\.\d)").unwrap();

        let body = response.body().await.unwrap();
        let buffer = String::from_utf8(body.to_vec()).unwrap();
        let matches = re.captures(&buffer).unwrap();

        let version = matches.get(1).unwrap();
        assert_eq!(
            version.as_str(),
            format!("v{}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
        );
        */
    }
}
