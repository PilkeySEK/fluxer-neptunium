#[cfg(feature = "rate-limiting")]
use crate::endpoints::MethodExt;
#[cfg(feature = "rate-limiting")]
use crate::ratelimiting::RateLimiter;
use bon::Builder;
use reqwest::{IntoUrl, Method, StatusCode};
use serde_json::Deserializer;

use crate::{
    BASE_USER_AGENT, DEFAULT_API_BASE_URL, VERSION,
    endpoints::{Endpoint, ExecuteEndpointRequestError, ResponseBody},
};

/// Defines the token prefix:
/// - `Bot`: "Bot"
/// - `User`: *(no prefix)*
/// - `Bearer`: "Bearer"
///
/// **Note:** To use the user API, enable the `user_api` feature.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum TokenType {
    #[default]
    Bot,
    User,
    Bearer,
}

#[derive(Debug, Builder)]
pub struct HttpClient {
    /// Change this if you want to make requests to a different instance.
    #[builder(into, default = DEFAULT_API_BASE_URL.to_owned())]
    pub api_base_url: String,
    #[builder(into)]
    pub token: zeroize::Zeroizing<String>,
    #[builder(default = TokenType::Bot)]
    pub token_type: TokenType,
    #[builder(skip = reqwest::Client::default())]
    pub(crate) reqwest_client: reqwest::Client,
    /// The user agent of the bot that should be sent (e.g.: `ReactionRolesBot/1.2.5 (+user#1234)`).
    #[builder(into)]
    pub bot_user_agent: Option<String>,
    /// Set to an empty string to disable.
    #[builder(into, default = format!("{}/{} (+{})", BASE_USER_AGENT.0, VERSION, BASE_USER_AGENT.1))]
    pub library_user_agent: String,
    #[cfg(feature = "rate-limiting")]
    #[builder(skip = RateLimiter::new(HttpClient::DEFAULT_GLOBAL_RATE_LIMIT))]
    pub(crate) rate_limiter: RateLimiter,
    /// How many times the client should retry if an error is received that is
    /// likely due to network conditions or rate limits. Set to 0 for no retries.
    #[builder(default = 3)]
    pub retry_times: usize,
}

impl HttpClient {
    /// The Fluxer global rate limit. This applies to all users unless they have a private `HIGH_GLOBAL_RATE_LIMIT` flag set.
    ///
    /// [Source](https://github.com/fluxerapp/fluxer/blob/ee1f27fe1a372b5291aead8042944afd706bf5db/packages/api/src/middleware/RateLimitMiddleware.tsx#L72).
    #[cfg(feature = "rate-limiting")]
    const DEFAULT_GLOBAL_RATE_LIMIT: u16 = 50;

    #[must_use]
    pub fn new(token: String) -> Self {
        Self::builder().token(token).build()
    }

    /// Since the user agent is composed of both the bot's user agent (if set)
    /// and the library user agent (set by default), this will create the full user
    /// agent string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use neptunium_http::client::HttpClient;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let client = HttpClient::builder()
    ///     .token("<the bot token>".to_string())
    ///     .library_user_agent("somethingelse/0.1.2 (+user#1234)")
    ///     .bot_user_agent("MyBot/1.0.0 (+user#1234)")
    ///     .build();
    ///
    /// assert_eq!(
    ///     client.get_full_user_agent(),
    ///     "MyBot/1.0.0 (+user#1234) somethingelse/0.1.2 (+user#1234)"
    /// );
    /// # }
    /// ```
    #[must_use]
    pub fn get_full_user_agent(&self) -> String {
        format!(
            "{}{}",
            if let Some(bot_user_agent) = &self.bot_user_agent {
                format!("{bot_user_agent} ")
            } else {
                String::new()
            },
            self.library_user_agent
        )
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
            tracing::trace!("Sending request: {:?}", request);
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
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)
                        .map_err(|e| ExecuteEndpointRequestError::DeserializationError(e, body))?;
                    (
                        Err(Box::new(ExecuteEndpointRequestError::BadRequest(api_error))),
                        false,
                    )
                }
                StatusCode::UNAUTHORIZED => {
                    let body = String::from_utf8(response.bytes().await?.to_vec())
                        .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                    let mut deserializer = Deserializer::from_str(&body);
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)
                        .map_err(|e| ExecuteEndpointRequestError::DeserializationError(e, body))?;
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
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)
                        .map_err(|e| ExecuteEndpointRequestError::DeserializationError(e, body))?;
                    (
                        Err(Box::new(ExecuteEndpointRequestError::NotFound(api_error))),
                        true,
                    )
                }
                StatusCode::FORBIDDEN => {
                    let body = String::from_utf8(response.bytes().await?.to_vec())
                        .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                    let mut deserializer = Deserializer::from_str(&body);
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)
                        .map_err(|e| ExecuteEndpointRequestError::DeserializationError(e, body))?;
                    (
                        Err(Box::new(ExecuteEndpointRequestError::Forbidden(api_error))),
                        false,
                    )
                }
                StatusCode::INTERNAL_SERVER_ERROR => {
                    let body = String::from_utf8(response.bytes().await?.to_vec())
                        .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                    let mut deserializer = Deserializer::from_str(&body);
                    let api_error = serde_path_to_error::deserialize(&mut deserializer)
                        .map_err(|e| ExecuteEndpointRequestError::DeserializationError(e, body))?;
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
                    let api_error = serde_path_to_error::deserialize(&mut deserializer).ok();
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

    /// Upload a file using the S3 API.
    ///
    /// # Errors
    /// Returns an error if executing the request fails or the response status is not a success response.
    pub async fn upload_file_s3(
        &self,
        url: impl IntoUrl,
        file_bytes: Vec<u8>,
    ) -> Result<(), Box<ExecuteEndpointRequestError>> {
        let response = self
            .reqwest_client
            .request(Method::PUT, url)
            .body(file_bytes)
            // TODO: Is this header needed?
            .header("Content-Type", "application/octet-stream")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Box::new(ExecuteEndpointRequestError::ResponseNotOk(
                response,
            )));
        }
        Ok(())
    }
}
