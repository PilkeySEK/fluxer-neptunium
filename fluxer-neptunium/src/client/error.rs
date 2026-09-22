use std::string::FromUtf8Error;

use neptunium_gateway::session::{ConnectError, SessionError};
use neptunium_http::{
    endpoints::ExecuteEndpointRequestError,
    error::{ApiErrorResponse, ApiRateLimitedResponse},
};
use tokio::task::JoinError;
use tokio_tungstenite::tungstenite;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("network error: {0}")]
    NetworkError(tungstenite::Error),
    #[error("failed to parse: {}, input: \"{}\"", .0, .1.trim())]
    ParseError(serde_path_to_error::Error<serde_json::Error>, String),
    #[error("the session is invalid")]
    SessionInvalidated,
    #[error("error sending HTTP request: {0}")]
    HttpRequestError(reqwest::Error),
    #[error("API did not respond OK: {0:?}")]
    HttpStatusNotOk(reqwest::Response),
    #[error("rate limited from API: {0:?}")]
    HttpRateLimited(ApiRateLimitedResponse),
    #[error("bad request from API: {0:?}")]
    HttpBadRequest(ApiErrorResponse),
    #[error("unauthorized from API: {0:?}")]
    HttpUnauthorized(ApiErrorResponse),
    #[error("forbidden from API: {0:?}")]
    HttpForbidden(ApiErrorResponse),
    #[error("not found from API: {0:?}")]
    HttpNotFound(ApiErrorResponse),
    #[error("internal server error from API: {0:?}")]
    HttpInternalServerError(ApiErrorResponse),
    #[error("invalid response: \"{0}\"")]
    HttpInvalidResponse(String),
    #[error("the client has stopped")]
    ClientNotPresent,
    #[error("{0}")]
    JoinError(JoinError),
    #[error("gateway session error: {0}")]
    GatewaySessionError(SessionError),
    #[error("error connecting to gateway: {0}")]
    GatewayConnectError(ConnectError),
    #[error("received non-utf8 bytes: {0}")]
    NonUtf8Bytes(FromUtf8Error),
    #[error("unexpected data received: {0}")]
    UnexpectedDataReceived(String),
    #[error("session not present")]
    SessionNotPresent,
}

impl From<tungstenite::Error> for ClientError {
    fn from(value: tungstenite::Error) -> Self {
        Self::NetworkError(value)
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(value: reqwest::Error) -> Self {
        Self::HttpRequestError(value)
    }
}

impl From<Box<ClientError>> for ClientError {
    fn from(value: Box<ClientError>) -> Self {
        *value
    }
}

impl From<ExecuteEndpointRequestError> for ClientError {
    fn from(value: ExecuteEndpointRequestError) -> Self {
        match value {
            ExecuteEndpointRequestError::DeserializationError(e, input) => {
                ClientError::ParseError(e, input)
            }
            ExecuteEndpointRequestError::NetworkError(e) => ClientError::HttpRequestError(e),
            ExecuteEndpointRequestError::NonUtf8Bytes(e) => ClientError::NonUtf8Bytes(e),
            ExecuteEndpointRequestError::ResponseNotOk(response) => {
                ClientError::HttpStatusNotOk(response)
            }
            ExecuteEndpointRequestError::NotFound(e) => ClientError::HttpNotFound(e),
            ExecuteEndpointRequestError::BadRequest(e) => ClientError::HttpBadRequest(e),
            ExecuteEndpointRequestError::Forbidden(e) => ClientError::HttpForbidden(e),
            ExecuteEndpointRequestError::RateLimited(e) => ClientError::HttpRateLimited(e),
            ExecuteEndpointRequestError::InternalServerError(e) => {
                ClientError::HttpInternalServerError(e)
            }
            ExecuteEndpointRequestError::Unauthorized(e) => ClientError::HttpUnauthorized(e),
        }
    }
}

impl From<Box<ExecuteEndpointRequestError>> for ClientError {
    fn from(value: Box<ExecuteEndpointRequestError>) -> Self {
        Self::from(*value)
    }
}
