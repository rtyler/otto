/**!
 * The auctioneer main model
 */
extern crate actix;
extern crate actix_http;
extern crate actix_web;
extern crate pretty_env_logger;

use actix_web::{middleware, web};
use actix_web::{App, HttpResponse, HttpServer};
use log::debug;

use otto_eventbus::client::*;

/**
 * The index handler for the root of the Auctioneer web interface
 */
async fn route_index() -> HttpResponse {
    HttpResponse::Ok().body("Auctioneer")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let client = connect("http://127.0.0.1:8000/ws/").await;
    debug!("Client created: {:?}", client);

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
        let srv = test::start(move || App::new().route("/", web::get().to(route_index)));

        let req = srv.get("/");
        let response = req.send().await.unwrap();
        assert!(response.status().is_success());
    }
}
