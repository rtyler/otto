/*
 * The control module handles all the agent<->step control messages
 */

use async_std::channel::Sender;
use log::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Request {
    Terminate,
}

/**
 * A simple struct to carry state around for control request handlers
 */
#[derive(Clone, Debug)]
struct State {
    sender: Sender<Request>,
}

async fn handle_request(mut req: tide::Request<State>) -> tide::Result {
    let message: Request = req.body_json().await?;
    debug!("Receiving control request on agent sock: {:#?}", message);
    req.state().sender.send(message).await;
    Ok("{}".into())
}

pub async fn run(pipeline: Uuid, sender: Sender<Request>) -> tide::Result<()> {
    info!("Starting the agent control server");
    let sock = agent_socket(&pipeline);
    let state = State { sender };
    let mut app = tide::with_state(state);

    app.at("/")
        .get(|_| async { Ok(format!("Otto Agent v{}", env!["CARGO_PKG_VERSION"])) });
    app.at("/control").post(handle_request);

    if let Err(e) = std::fs::remove_file(&sock) {
        warn!(
            "Failed while trying to remove any previous {:?}, this might be okay: {}",
            &sock, e
        );
    }

    app.listen(format!("http+unix://{}", sock.to_string_lossy()))
        .await?;

    Ok(())
}

/**
 * Return a string representing the absolute path of this agent's control socket
 */
pub fn agent_socket(pipeline: &Uuid) -> std::path::PathBuf {
    let uuid = pipeline.to_hyphenated().to_string();
    std::env::temp_dir()
        .join(format!("{}-agent.sock", uuid))
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_sock() {
        let uuid = uuid::Uuid::new_v4();
        let buf = agent_socket(&uuid);
        assert!(buf.to_string_lossy().ends_with("agent.sock"));
    }
}
