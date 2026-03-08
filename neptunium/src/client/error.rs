use fluxer_gateway::shard::EventReceiveError;
use fluxer_model::gateway::event::gateway::GatewayEvent;
use tokio_tungstenite::tungstenite::{self, protocol::CloseFrame};

#[derive(Debug)]
pub struct Error {
    kind: ClientErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ClientErrorKind) -> Self {
        Self { kind }
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ClientErrorKind::NetworkError(e) => f.write_fmt(format_args!("Network error: {e}")),
            ClientErrorKind::ParseError(e) => f.write_fmt(format_args!("Parse error: {e}")),
            ClientErrorKind::UnexpectedEventReceived(event) => {
                f.write_fmt(format_args!("Unexpected event received: {event:?}"))
            }
            ClientErrorKind::UnsupportedMessageEncoding => {
                f.write_str("Unsupported message encoding")
            }
            ClientErrorKind::ConnectionClosed(frame) => match frame {
                Some(frame) => f.write_fmt(format_args!(
                    "Connection closed: code={}, reason=\"{}\"",
                    frame.code, frame.reason
                )),
                None => f.write_fmt(format_args!("Connection closed, no close frame present")),
            },
        }
    }
}

#[derive(Debug)]
pub enum ClientErrorKind {
    NetworkError(tungstenite::Error),
    ParseError(serde_json::Error),
    UnsupportedMessageEncoding,
    UnexpectedEventReceived(Box<GatewayEvent>),
    ConnectionClosed(Option<CloseFrame>),
}

impl From<tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Self {
        Self {
            kind: ClientErrorKind::NetworkError(value),
        }
    }
}

impl From<EventReceiveError> for Error {
    fn from(value: EventReceiveError) -> Self {
        Self {
            kind: match value {
                EventReceiveError::ParseError(e) => ClientErrorKind::ParseError(e),
                EventReceiveError::TungsteniteError(e) => ClientErrorKind::NetworkError(e),
                EventReceiveError::UnsupportedMessageEncoding => {
                    ClientErrorKind::UnsupportedMessageEncoding
                }
                EventReceiveError::Closed(frame) => ClientErrorKind::ConnectionClosed(frame),
            },
        }
    }
}
