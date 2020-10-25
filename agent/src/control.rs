/*
 * The control module handles all the agent<->step control messages
 */

use log::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum Request {
    Terminate,
}

async fn handle_request(mut req: tide::Request<()>) -> tide::Result {
    let message: Request = req.body_json().await?;
    debug!("Handling control request: {:#?}", message);
    Ok("{}".into())
}

pub async fn run() -> tide::Result<()> {
    info!("Starting the agent control server");
    let sock = "agent.sock";
    let mut app = tide::new();

    app.at("/").get(|_| async { Ok(format!("Otto Agent v{}", env!["CARGO_PKG_VERSION"])) });
    app.at("/control").post(handle_request);

    if let Err(e) = std::fs::remove_file(&sock) {
        warn!("Failed while trying to remove any previous {}, this might be okay", &sock);
    }

    app.listen(format!("http+unix://{}", sock)).await?;
    Ok(())
}
