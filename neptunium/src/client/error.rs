use fluxer_gateway::shard::EventReceiveError;
use fluxer_model::gateway::event::gateway::GatewayEvent;
use neptunium_http::{
    endpoints::ExecuteEndpointRequestError,
    responses::error::{ApiErrorResponse, ApiRateLimitedResponse},
};
use tokio_tungstenite::tungstenite::{self, protocol::CloseFrame};

#[derive(Debug)]
pub struct Error {
    kind: ClientErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ClientErrorKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> &ClientErrorKind {
        &self.kind
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
            ClientErrorKind::SessionInvalidated => f.write_str("Session invalidated"),
            ClientErrorKind::HttpRequestError(e) => f.write_fmt(format_args!("HTTP error: {e}")),
            ClientErrorKind::HttpStatusNotOk(response) => f.write_fmt(format_args!(
                "HTTP error: The server did not respond OK: {response:?}"
            )),
            // TODO: Improve these error messages
            ClientErrorKind::HttpNotFound(err) => {
                f.write_fmt(format_args!("API Error: Not Found: {err:?}"))
            }
            ClientErrorKind::HttpBadRequest(err) => {
                f.write_fmt(format_args!("API Error: Bad Request: {err:?}"))
            }
            ClientErrorKind::HttpForbidden(err) => {
                f.write_fmt(format_args!("API Error: Forbidden: {err:?}"))
            }
            ClientErrorKind::HttpRateLimited(err) => {
                f.write_fmt(format_args!("API Error: Rate Limited: {err:?}"))
            }
            ClientErrorKind::HttpInternalServerError(err) => {
                f.write_fmt(format_args!("API Error: Internal Server Error: {err:?}"))
            }
            ClientErrorKind::HttpUnauthorized(err) => {
                f.write_fmt(format_args!("API Error: Unauthorized: {err:?}"))
            }
        }
    }
}

impl From<ExecuteEndpointRequestError> for Error {
    fn from(value: ExecuteEndpointRequestError) -> Self {
        Self::new(match value {
            ExecuteEndpointRequestError::DeserializationError(e) => ClientErrorKind::ParseError(e),
            ExecuteEndpointRequestError::NetworkError(e) => ClientErrorKind::HttpRequestError(e),
            ExecuteEndpointRequestError::NonUtf8Bytes(_) => {
                ClientErrorKind::UnsupportedMessageEncoding
            }
            ExecuteEndpointRequestError::ResponseNotOk(response) => {
                ClientErrorKind::HttpStatusNotOk(response)
            }
            ExecuteEndpointRequestError::NotFound(err) => ClientErrorKind::HttpNotFound(err),
            ExecuteEndpointRequestError::BadRequest(err) => ClientErrorKind::HttpBadRequest(err),
            ExecuteEndpointRequestError::Forbidden(err) => ClientErrorKind::HttpForbidden(err),
            ExecuteEndpointRequestError::RateLimited(err) => ClientErrorKind::HttpRateLimited(err),
            ExecuteEndpointRequestError::InternalServerError(err) => {
                ClientErrorKind::HttpInternalServerError(err)
            }
            ExecuteEndpointRequestError::Unauthorized(err) => {
                ClientErrorKind::HttpUnauthorized(err)
            }
        })
    }
}
impl From<Box<ExecuteEndpointRequestError>> for Error {
    fn from(value: Box<ExecuteEndpointRequestError>) -> Self {
        Self::from(*value)
    }
}

#[derive(Debug)]
pub enum ClientErrorKind {
    NetworkError(tungstenite::Error),
    ParseError(serde_path_to_error::Error<serde_json::Error>),
    UnsupportedMessageEncoding,
    UnexpectedEventReceived(Box<GatewayEvent>),
    ConnectionClosed(Option<CloseFrame>),
    SessionInvalidated,
    HttpRequestError(reqwest::Error),
    HttpStatusNotOk(reqwest::Response),
    HttpRateLimited(ApiRateLimitedResponse),
    HttpBadRequest(ApiErrorResponse),
    HttpUnauthorized(ApiErrorResponse),
    HttpForbidden(ApiErrorResponse),
    HttpNotFound(ApiErrorResponse),
    HttpInternalServerError(ApiErrorResponse),
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

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self {
            kind: ClientErrorKind::HttpRequestError(value),
        }
    }
}
