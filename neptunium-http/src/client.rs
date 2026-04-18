#[cfg(feature = "rate-limiting")]
use crate::endpoints::MethodExt;
#[cfg(feature = "rate-limiting")]
use crate::ratelimiting::RateLimiter;
use reqwest::StatusCode;
use serde_json::Deserializer;

use crate::{
    DEFAULT_API_BASE_URL, DEFAULT_USER_AGENT, VERSION,
    endpoints::{Endpoint, ExecuteEndpointRequestError, ResponseBody},
};

/// Bot tokens have `Bot ` prefix, user tokens do not.
#[cfg(feature = "user_api")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum TokenType {
    #[default]
    Bot,
    User,
}

#[derive(Debug)]
pub struct HttpClient {
    pub(crate) api_base_url: String,
    pub(crate) token: zeroize::Zeroizing<String>,
    #[cfg(feature = "user_api")]
    pub token_type: TokenType,
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) user_agent: String,
    #[cfg(feature = "rate-limiting")]
    pub(crate) rate_limiter: RateLimiter,
    /// How many times the client should retry if an error is received that is
    /// likely due to network conditions or rate limits. Set to 0 for no retries.
    pub retry_times: usize,
}

impl HttpClient {
    /// The Fluxer global rate limit. This applies to all users unless they have a private `HIGH_GLOBAL_RATE_LIMIT` flag set.
    ///
    /// [Source](https://github.com/fluxerapp/fluxer/blob/ee1f27fe1a372b5291aead8042944afd706bf5db/packages/api/src/middleware/RateLimitMiddleware.tsx#L72)
    #[cfg(feature = "rate-limiting")]
    const DEFAULT_GLOBAL_RATE_LIMIT: u16 = 50;
    #[must_use]
    pub fn new(token: String, #[cfg(feature = "user_api")] token_type: TokenType) -> Self {
        Self {
            api_base_url: DEFAULT_API_BASE_URL.to_owned(),
            reqwest_client: reqwest::Client::default(),
            token: zeroize::Zeroizing::new(token),
            #[cfg(feature = "user_api")]
            token_type,
            user_agent: format!("{DEFAULT_USER_AGENT}/{VERSION}"),
            #[cfg(feature = "rate-limiting")]
            rate_limiter: RateLimiter::new(Self::DEFAULT_GLOBAL_RATE_LIMIT),
            retry_times: 3,
        }
    }

    pub fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = user_agent;
    }

    pub fn set_api_base_url(&mut self, url: String) {
        self.api_base_url = url;
    }

    /// Send a request to the specified endpoint, returning the result.
    /// # Errors
    /// Returns an error if the response is not what is expected by the given endpoint or if a network error occurs.
    #[expect(clippy::too_many_lines)]
    pub async fn execute<T: Endpoint + Send>(
        &self,
        endpoint: T,
    ) -> Result<T::Response, Box<ExecuteEndpointRequestError>> {
        let mut current_try = 0;
        loop {
            let request = endpoint.clone().into_request();
            // Disable rate limiting in debug mode because of twilight-http-ratelimiting bug
            #[cfg(feature = "rate-limiting")]
            let permit = 'rate_limiting_blk: {
                let path_for_rate_limiter = request.path.strip_prefix('/').unwrap_or(&request.path);
                let Some(method_for_rate_limiter) =
                    request.method.clone().into_rate_limiter_method()
                else {
                    tracing::warn!(
                        "Could not convert reqwest method {} to method for rate limiter.",
                        request.method
                    );
                    break 'rate_limiting_blk None;
                };
                Some(
                    self.rate_limiter
                        .acquire(crate::ratelimiting::Endpoint {
                            method: method_for_rate_limiter,
                            path: path_for_rate_limiter.to_owned(),
                        })
                        .await,
                )
            };
            let response = match request.execute(self).await {
                Ok(response) => response,
                Err(e) => {
                    current_try += 1;
                    if current_try > self.retry_times {
                        break Err(e.into());
                    }
                    tracing::debug!("Error sending request, retrying: {e}");
                    continue;
                }
            };
            tracing::trace!("API response: {:?}", response);

            #[cfg(feature = "rate-limiting")]
            if let Some(permit) = permit {
                use crate::ratelimiting::RateLimitHeaders;

                use crate::endpoints::RateLimitHeadersExt;

                let headers = RateLimitHeaders::from_response(&response);
                permit.complete(headers);
            }

            // TODO: Move this match logic to a new function
            let (result, should_retry) = match response.status() {
                StatusCode::OK | StatusCode::NO_CONTENT => {
                    let body = response.bytes().await?.to_vec();
                    tracing::trace!("API response body: {:?}", body);
                    (ResponseBody::deserialize(body), true)
                }
                StatusCode::BAD_REQUEST => {
                    let body = String::from_utf8(response.bytes().await?.to_vec())
                        .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                    let mut deserializer = Deserializer::from_str(&body);
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                    (
                        Err(Box::new(ExecuteEndpointRequestError::BadRequest(api_error))),
                        false,
                    )
                }
                StatusCode::UNAUTHORIZED => {
                    let body = String::from_utf8(response.bytes().await?.to_vec())
                        .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                    let mut deserializer = Deserializer::from_str(&body);
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                    (
                        Err(Box::new(ExecuteEndpointRequestError::Unauthorized(
                            api_error,
                        ))),
                        false,
                    )
                }
                StatusCode::NOT_FOUND => {
                    let body = String::from_utf8(response.bytes().await?.to_vec())
                        .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                    let mut deserializer = Deserializer::from_str(&body);
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                    (
                        Err(Box::new(ExecuteEndpointRequestError::NotFound(api_error))),
                        true,
                    )
                }
                StatusCode::FORBIDDEN => {
                    let body = String::from_utf8(response.bytes().await?.to_vec())
                        .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                    let mut deserializer = Deserializer::from_str(&body);
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                    (
                        Err(Box::new(ExecuteEndpointRequestError::Forbidden(api_error))),
                        false,
                    )
                }
                StatusCode::INTERNAL_SERVER_ERROR => {
                    let body = String::from_utf8(response.bytes().await?.to_vec())
                        .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                    let mut deserializer = Deserializer::from_str(&body);
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                    (
                        Err(Box::new(ExecuteEndpointRequestError::InternalServerError(
                            api_error,
                        ))),
                        true,
                    )
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    let body = String::from_utf8(response.bytes().await?.to_vec())
                        .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                    let mut deserializer = Deserializer::from_str(&body);
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                    (
                        Err(Box::new(ExecuteEndpointRequestError::RateLimited(
                            api_error,
                        ))),
                        true,
                    )
                }
                _ => (
                    Err(Box::new(ExecuteEndpointRequestError::ResponseNotOk(
                        response,
                    ))),
                    true,
                ),
            };

            match result {
                Ok(value) => break Ok(value),
                Err(e) => {
                    if !should_retry {
                        tracing::debug!("Should not retry request, returning error.");
                        break Err(e);
                    }
                    current_try += 1;
                    if current_try > self.retry_times {
                        tracing::debug!("No retries left, returning error.");
                        break Err(e);
                    }
                    tracing::debug!("API returned error, retrying: {e:?}");
                }
            }
        }
    }
}
