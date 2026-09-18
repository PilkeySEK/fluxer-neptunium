use std::string::FromUtf8Error;

use thiserror::Error;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("WebSocket error: {0}")]
    WebSocket(tokio_tungstenite::tungstenite::Error),
    // #[error(
    //     "gave up on reconnecting after {num_tries} {}",
    //     if *num_tries == 1 { "attempt" } else { "attempts" }
    // )]
    // ReconnectFailed { num_tries: usize },
    #[error("failed to deserialize: {0}")]
    Deserialize(serde_json::Error),
}

impl From<tokio_tungstenite::tungstenite::Error> for SessionError {
    fn from(value: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WebSocket(value)
    }
}
