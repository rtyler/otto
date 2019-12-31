/**
 * The main module for otto-eventbus simply sets up the responders and the main
 * server loop that the eventbus uses.
 */
#[allow(unused_imports)]

extern crate config;
extern crate mime;
extern crate log;
extern crate num_cpus;
extern crate pretty_env_logger;
extern crate tide;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate serde_json;

use bastion::prelude::*;
use chrono::Local;
use handlebars::Handlebars;
use log::{info, trace};
use mime::Mime;
use tide::*;

use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use otto_eventbus::Command;

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

async fn index(_req: Request<()>) -> Response {
    let hb = Handlebars::new();
    let data = json!({
        "version" : option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
    });

    let template = Templates::get("index.html").expect("Failed to fetch embedded index.html template");
    let body = hb
        .render_template(std::str::from_utf8(template.as_ref()).unwrap(), &data)
        .expect("Failed to render the index.html template!");

    Response::new(200).body_string(body).set_mime(mime::TEXT_HTML)
}

fn main() {
    pretty_env_logger::init();
    Bastion::init();

    let supervisor = Bastion::supervisor(|sp| sp.with_strategy(SupervisionStrategy::OneForOne))
        .expect("Couldn't create the supervisor.");
    /*
     * Wrapping the start of Gotham with a Bastion supervisor.
     *
     * This doesn't necessarily help _too_ much since Gotham is using Tokio underneath the covers
     * in order to handle requests.
     *
     * My ideal approach would be to have one supervisor group with server listeners and another
     * with handlers, and let Bastion drive the whole thing by by sending messages over to handlers
     * when requests come in.
     */
    supervisor.children(|children: Children| {
        children.with_exec(move |_ctx: BastionContext| {
            async move {
                let mut app = tide::new();
                app.at("/").get(index);
                app.listen("127.0.0.1:8000").await.expect("Failed to listen");
                Ok(())
            }
        })
    })
    .expect("Could not start the Tide worker children");

    let ws_workers = supervisor.children(|children: Children| {
        children
            .with_redundancy(num_cpus::get())
            .with_exec(move |ctx: BastionContext| {
                async move {
                    // Start receiving work
                    loop {
                        msg! { ctx.recv().await?,
                            msg: u64 =!> {
                                let data: u64 = msg.wrapping_mul(2);
                                println!("Child doubled the value of {} and gave {}", msg, data); // true
                                let _ = answer!(ctx, data);
                            };
                            _: _ => ();
                        }
                    }
                }
            })
    }).expect("Could not start the WebSocket worker children");
    let ws_workers = Arc::new(ws_workers);

    /*
     * At the moment there's not a clear way to integrate Tide and Tungstenite together such that
     * you don't need to listen on two ports.
     *
     * See:
     *   - https://github.com/snapview/tungstenite-rs/issues/70
     *   - https://github.com/http-rs/tide/issues/67
     */
    supervisor.children(|children: Children| {
        children.with_exec(move |_ctx: BastionContext| {
            ws_workers.clone();
            async move {
                let server = TcpListener::bind("127.0.0.1:8081").unwrap();
                Ok(())
            }
        })
    })
    .expect("Could not start the tungstenite server children");

    Bastion::start();
    Bastion::block_until_stopped();

}


// * ws_index is the handler for all websocket connections, all it is responsible for doing is
// * handling the inbound connection and creating a new WSClient for the connection
// */
//async fn ws_index(
//    r: HttpRequest,
//    stream: web::Payload,
//    state: web::Data<AppState>,
//) -> Result<HttpResponse, Error> {
//    let actor = connection::WSClient::new(state.bus.clone());
//    let res = ws::start(actor, &r, stream);
//    trace!("{:?}", res.as_ref().unwrap());
//    res
//}

//#[actix_rt::main]
//async fn main() -> std::io::Result<()> {
//    pretty_env_logger::init();
//
//    let embedded_settings = Static::get("eventbus.yml").unwrap();
//    let defaults = std::str::from_utf8(embedded_settings.as_ref()).unwrap();
//    /*
//     * Load our settings in the priority order of:
//     *
//     *   - built-in defaults
//     *   - yaml file
//     *   - environment variables
//     *
//     * Each layer overriding properties from the last
//     */
//    let mut settings = config::Config::default();
//    settings
//        .merge(config::File::from_str(defaults, config::FileFormat::Yaml))
//        .unwrap()
//        .merge(config::File::with_name("eventbus"))
//        .unwrap()
//        .merge(config::Environment::with_prefix("OTTO_EB"))
//        .unwrap();
//
//    let motd: String = settings
//        .get("motd")
//        .expect("Configuration had no `motd` setting");
//
//    info!("motd: {}", motd);
//
//    let stateless = settings
//        .get::<Vec<String>>("channels.stateless")
//        .expect("Failed to load `channels.stateless` configuration, which must be an array");
//    let stateful = settings
//        .get::<Vec<String>>("channels.stateful")
//        .expect("Failed to load `channels.stateful` configuration, which must be an array");
//
//    let events = eventbus::EventBus::with_channels(stateless, stateful).start();
//    let bus = events.clone();
//
//    thread::spawn(move || loop {
//        let pulse = format!("heartbeat {}", Local::now());
//        trace!("sending pulse: {}", pulse);
//        let event = eventbus::Event {
//            e: Arc::new(Command::Heartbeat),
//            channel: "all".to_string(),
//        };
//        bus.do_send(event);
//        let seconds = settings
//            .get("heartbeat")
//            .expect("Invalid `heartbeat` configuration, must be an integer");
//        thread::sleep(Duration::from_secs(seconds));
//    });
//
//    let state = AppState {
//        bus: events,
//        hb: Arc::new(Handlebars::new()),
//    };
//    let wd = web::Data::new(state);
//
//    HttpServer::new(move || {
//        App::new()
//            .app_data(wd.clone())
//            .wrap(middleware::Compress::default())
//            .wrap(middleware::Logger::default())
//            .route("/", web::get().to(index))
//            .route("/ws/", web::get().to(ws_index))
//    })
//    .bind("127.0.0.1:8000")?
//    .run()
//    .await
//}
