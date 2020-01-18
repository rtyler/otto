/**!
 * The auctioneer main model
 */
#[deny(unsafe_code)]

extern crate pretty_env_logger;
extern crate tokio;
extern crate warp;

use log::debug;
use warp::{Filter, Rejection};

pub fn index_filter() -> impl Filter<Extract = (&'static str,), Error = Rejection> + Clone {
    warp::path::end().map(|| "Index page")
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = index_filter();
    warp::serve(routes).run(([127, 0, 0, 1], 8001)).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    async fn test_index() {
        let index = index_filter();
        let response = request().method("GET").path("/").reply(&index).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body(), "Index page");
    }
}
