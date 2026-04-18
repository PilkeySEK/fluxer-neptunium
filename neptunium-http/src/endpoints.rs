use std::string::FromUtf8Error;

#[cfg(feature = "rate-limiting")]
use crate::ratelimiting::RateLimitHeaders;
#[cfg(feature = "rate-limiting")]
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::{
    error::{ApiErrorResponse, ApiRateLimitedResponse},
    request::Request,
};

pub mod channel;
pub mod gateway;
pub mod guild;
pub mod invites;
pub mod meta;
#[cfg(feature = "user_api")]
pub mod saved_media;
pub mod users;
pub mod webhooks;

impl<T: DeserializeOwned> ResponseBody for T {
    fn deserialize(bytes: Vec<u8>) -> Result<Self, Box<ExecuteEndpointRequestError>> {
        if bytes.is_empty() {
            let mut deserializer = serde_json::Deserializer::from_str("null");
            Ok(serde_path_to_error::deserialize(&mut deserializer)?)
        } else {
            let s = String::from_utf8(bytes).map_err(ExecuteEndpointRequestError::NonUtf8Bytes)?;
            let mut deserializer = serde_json::Deserializer::from_str(&s);
            Ok(serde_path_to_error::deserialize(&mut deserializer)?)
        }
    }
}

pub trait ResponseBody: Sized {
    /// Deserialize, given the response body bytes.
    /// # Errors
    /// Returns an error if deserializing failed.
    fn deserialize(bytes: Vec<u8>) -> Result<Self, Box<ExecuteEndpointRequestError>>;
}

// Trait bounds to make sure all endpoints implement these, they aren't technically required.
pub trait Endpoint: Clone + std::fmt::Debug {
    type Response: ResponseBody;
    fn into_request(self) -> Request;
    // async fn deserialize_response(
    //     response: reqwest::Response,
    // ) -> Result<Self::Response, ExecuteEndpointRequestError> {
    //     if response.status() == StatusCode::OK || response.status() == StatusCode::NO_CONTENT {
    //         if size_of::<Self::Response>() == 0 {
    //             let mut deserializer = serde_json::Deserializer::from_slice(&[]);
    //             Ok(serde_path_to_error::deserialize(&mut deserializer)?)
    //         } else {
    //             let body = String::from_utf8(response.bytes().await?.to_vec())
    //                 .map_err(ExecuteEndpointRequestError::NonUtf8Bytes)?;
    //             let mut deserializer = serde_json::Deserializer::from_str(&body);
    //             Ok(serde_path_to_error::deserialize(&mut deserializer)?)
    //         }
    //     } else {
    //         Err(ExecuteEndpointRequestError::ResponseNotOk(response))
    //     }
    // }
}

#[derive(Debug)]
pub enum ExecuteEndpointRequestError {
    NetworkError(reqwest::Error),
    ResponseNotOk(reqwest::Response),
    DeserializationError(serde_path_to_error::Error<serde_json::Error>),
    NonUtf8Bytes(FromUtf8Error),
    // TODO: Add fields to this and stuff.
    // Also need to actually implement rate limit handling
    // and waiting until the rate limit expires so that the
    // user doesn't need to worry about this.
    /// 429 Too Many Requests.
    RateLimited(ApiRateLimitedResponse),
    /// 400 Bad Request.
    BadRequest(ApiErrorResponse),
    /// 401 Unauthorized.
    Unauthorized(ApiErrorResponse),
    /// 403 Forbidden.
    Forbidden(ApiErrorResponse),
    /// 404 Not Found.
    NotFound(ApiErrorResponse),
    /// 500 Internal Server Error.
    InternalServerError(ApiErrorResponse),
}

impl From<reqwest::Error> for ExecuteEndpointRequestError {
    fn from(value: reqwest::Error) -> Self {
        Self::NetworkError(value)
    }
}

impl From<reqwest::Error> for Box<ExecuteEndpointRequestError> {
    fn from(value: reqwest::Error) -> Self {
        Box::new(ExecuteEndpointRequestError::NetworkError(value))
    }
}

impl From<serde_path_to_error::Error<serde_json::Error>> for Box<ExecuteEndpointRequestError> {
    fn from(value: serde_path_to_error::Error<serde_json::Error>) -> Self {
        Box::new(ExecuteEndpointRequestError::DeserializationError(value))
    }
}

impl From<serde_path_to_error::Error<serde_json::Error>> for ExecuteEndpointRequestError {
    fn from(value: serde_path_to_error::Error<serde_json::Error>) -> Self {
        Self::DeserializationError(value)
    }
}

#[cfg(feature = "rate-limiting")]
pub(crate) trait MethodExt {
    fn into_rate_limiter_method(self) -> Option<crate::ratelimiting::Method>;
}

#[cfg(feature = "rate-limiting")]
impl MethodExt for reqwest::Method {
    fn into_rate_limiter_method(self) -> Option<crate::ratelimiting::Method> {
        use crate::ratelimiting::Method as TwilightMethod;
        use reqwest::Method as ReqwestMethod;
        Some(match self {
            ReqwestMethod::GET => TwilightMethod::Get,
            ReqwestMethod::POST => TwilightMethod::Post,
            ReqwestMethod::PUT => TwilightMethod::Put,
            ReqwestMethod::PATCH => TwilightMethod::Patch,
            ReqwestMethod::DELETE => TwilightMethod::Delete,
            _ => return None,
        })
    }
}

#[cfg(feature = "rate-limiting")]
pub(crate) trait RateLimitHeadersExt: Sized {
    fn from_response(res: &Response) -> Option<Self>;
}

#[cfg(feature = "rate-limiting")]
impl RateLimitHeadersExt for RateLimitHeaders {
    fn from_response(res: &Response) -> Option<Self> {
        let headers = res.headers();
        Some(RateLimitHeaders {
            bucket: headers
                .get(RateLimitHeaders::BUCKET)?
                .to_str()
                .map_or(None, |value| Some(parse_bucket(value)))??,
            limit: headers
                .get(RateLimitHeaders::LIMIT)?
                .to_str()
                .ok()?
                .parse::<u16>()
                .ok()?,
            remaining: headers
                .get(RateLimitHeaders::REMAINING)?
                .to_str()
                .ok()?
                .parse::<u16>()
                .ok()?,
            reset_at: {
                use std::time::{Duration, Instant};
                let reset_after = headers
                    .get(RateLimitHeaders::RESET_AFTER)?
                    .to_str()
                    .ok()?
                    .parse::<f64>()
                    .ok()?;
                Instant::now() + Duration::from_secs_f64(reset_after)
            },
        })
    }
}

#[cfg(feature = "rate-limiting")]
fn parse_bucket(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    let mut vec = Vec::with_capacity(8);
    let mut chars = s.chars();
    for _ in 0..(s.len() / 2) {
        let chars_string = [chars.next()?, chars.next()?]
            .into_iter()
            .collect::<String>();
        vec.push(u8::from_str_radix(&chars_string, 16).ok()?);
    }
    Some(vec)
}
