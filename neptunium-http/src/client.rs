#[cfg(feature = "rate-limiting")]
use crate::endpoints::MethodExt;
#[cfg(feature = "rate-limiting")]
use crate::ratelimiting::RateLimiter;
use bon::Builder;
use reqwest::{IntoUrl, Method, StatusCode};
use serde_json::Deserializer;
use zeroize::Zeroizing;

use crate::{
    BASE_USER_AGENT, VERSION,
    endpoints::{
        Endpoint, ExecuteEndpointRequestError, ResponseBody,
        meta::{GetInstanceDiscoveryDocument, InstanceDiscoveryDocumentResponse},
    },
    request::RequestExecuteClientInfo,
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

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct HttpClientConfig {
    #[builder(into)]
    pub token: String,
    #[builder(default = 3)]
    pub max_request_retries: usize,
    #[builder(default)]
    pub token_type: TokenType,
    /// Set to an empty string to disable.
    #[builder(into, default = HttpClientConfig::default_user_agent())]
    pub user_agent: String,
    #[builder(default = crate::DEFAULT_INSTANCE_ROOT.to_owned())]
    pub instance_root: String,
}

impl HttpClientConfig {
    #[must_use]
    pub fn default_user_agent() -> String {
        format!("{}/{} (+{})", BASE_USER_AGENT.0, VERSION, BASE_USER_AGENT.1)
    }
}

#[derive(Debug)]
pub struct HttpClient {
    pub api_base_url: String,
    pub token: zeroize::Zeroizing<String>,
    pub token_type: TokenType,
    pub(crate) reqwest_client: reqwest::Client,
    // /// The user agent of the bot that should be sent (e.g.: `ReactionRolesBot/1.2.5 (+user#1234)`).
    // pub bot_user_agent: Option<String>,
    // /// Set to an empty string to disable.
    // pub library_user_agent: String,
    pub user_agent: Option<String>,
    #[cfg(feature = "rate-limiting")]
    pub(crate) rate_limiter: RateLimiter,
    /// How many times the client should retry if an error is received that is
    /// likely due to network conditions or rate limits. Set to 0 for no retries.
    pub retry_times: usize,
}

impl HttpClient {
    /// The Fluxer global rate limit. This applies to all users unless they have a private `HIGH_GLOBAL_RATE_LIMIT` flag set.
    ///
    /// [Source](https://github.com/fluxerapp/fluxer/blob/ee1f27fe1a372b5291aead8042944afd706bf5db/packages/api/src/middleware/RateLimitMiddleware.tsx#L72).
    #[cfg(feature = "rate-limiting")]
    const DEFAULT_GLOBAL_RATE_LIMIT: u16 = 50;

    // #[must_use]
    // pub fn new(token: String, api_base_url: String) -> Self {
    //     Self::builder()
    //         .token(token)
    //         .api_base_url(api_base_url)
    //         .build()
    // }

    /// Send a request to the specified endpoint, returning the result.
    /// # Errors
    /// Returns an error if the response is not what is expected by the given endpoint or if a network error occurs.
    pub async fn execute<T: Endpoint + Send>(
        &self,
        endpoint: T,
    ) -> Result<T::Response, ExecuteEndpointRequestError> {
        try_request(
            &self.api_base_url,
            endpoint.into_request(),
            &self.rate_limiter,
            self.retry_times,
            &self.reqwest_client,
            Some((&self.token, self.token_type)),
            self.user_agent.as_deref(),
        )
        .await
    }

    /// Upload a file using the S3 API.
    ///
    /// # Errors
    /// Returns an error if executing the request fails or the response status is not a success response.
    pub async fn upload_file_s3(
        &self,
        url: impl IntoUrl,
        file_bytes: Vec<u8>,
    ) -> Result<(), ExecuteEndpointRequestError> {
        let response = self
            .reqwest_client
            .request(Method::PUT, url)
            .body(file_bytes)
            // TODO: Is this header needed?
            .header("Content-Type", "application/octet-stream")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ExecuteEndpointRequestError::ResponseNotOk(Box::new(
                response,
            )));
        }
        Ok(())
    }

    #[must_use]
    pub fn new_with_api_base_url(config: HttpClientConfig, api_base_url: String) -> Self {
        Self {
            api_base_url,
            token: Zeroizing::new(config.token),
            token_type: config.token_type,
            reqwest_client: reqwest::Client::new(),
            rate_limiter: RateLimiter::new(Self::DEFAULT_GLOBAL_RATE_LIMIT),
            user_agent: if config.user_agent.is_empty() {
                None
            } else {
                Some(config.user_agent)
            },
            retry_times: config.max_request_retries,
        }
    }

    /// Will get required information from the instance discovery document.
    ///
    /// # Errors
    /// Returns an error if fetching the instance discovery document fails.
    pub async fn new(
        config: HttpClientConfig,
    ) -> Result<(Self, InstanceDiscoveryDocumentResponse), ExecuteEndpointRequestError> {
        let user_agent = if config.user_agent.is_empty() {
            None
        } else {
            Some(config.user_agent)
        };
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(Self::DEFAULT_GLOBAL_RATE_LIMIT);
        let req = GetInstanceDiscoveryDocument.into_request();
        let res: InstanceDiscoveryDocumentResponse = try_request(
            &config.instance_root,
            req,
            &rate_limiter,
            config.max_request_retries,
            &client,
            None,
            user_agent.as_deref(),
        )
        .await?;

        Ok((
            Self {
                api_base_url: res.endpoints.api_public.clone(),
                token: Zeroizing::new(config.token),
                token_type: config.token_type,
                reqwest_client: client,
                rate_limiter,
                user_agent,
                retry_times: config.max_request_retries,
            },
            res,
        ))
    }
}

#[expect(clippy::too_many_lines)]
async fn try_request<Res: ResponseBody>(
    base: &str,
    req: crate::request::Request,
    rate_limiter: &RateLimiter,
    max_retries: usize,
    client: &reqwest::Client,
    token_info: Option<(&str, TokenType)>,
    user_agent: Option<&str>,
) -> Result<Res, ExecuteEndpointRequestError> {
    let mut current_try = 0;
    loop {
        let request = req.clone();
        #[cfg(feature = "rate-limiting")]
        let permit = 'rate_limiting_blk: {
            let path_for_rate_limiter = request.path.strip_prefix('/').unwrap_or(&request.path);
            let Some(method_for_rate_limiter) = request.method.clone().into_rate_limiter_method()
            else {
                tracing::warn!(
                    "Could not convert reqwest method {} to method for rate limiter.",
                    request.method
                );
                break 'rate_limiting_blk None;
            };
            Some(
                rate_limiter
                    .acquire(crate::ratelimiting::Endpoint {
                        method: method_for_rate_limiter,
                        path: path_for_rate_limiter.to_owned(),
                    })
                    .await,
            )
        };
        tracing::trace!("Sending request: {:?}", request);
        let response = match request
            .execute(RequestExecuteClientInfo {
                reqwest_client: client,
                base_url: base,
                token_info,
                user_agent,
            })
            .await
        {
            Ok(response) => response,
            Err(e) => {
                current_try += 1;
                if current_try > max_retries {
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
            StatusCode::OK | StatusCode::NO_CONTENT | StatusCode::CREATED => {
                let body = response.bytes().await?.to_vec();
                tracing::trace!("API response body: {:?}", body);
                (ResponseBody::deserialize(body), true)
            }
            StatusCode::BAD_REQUEST => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(ExecuteEndpointRequestError::NonUtf8Bytes)?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)
                    .map_err(|e| ExecuteEndpointRequestError::DeserializationError(e, body))?;
                (
                    Err(ExecuteEndpointRequestError::BadRequest(api_error)),
                    false,
                )
            }
            StatusCode::UNAUTHORIZED => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(ExecuteEndpointRequestError::NonUtf8Bytes)?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)
                    .map_err(|e| ExecuteEndpointRequestError::DeserializationError(e, body))?;
                (
                    Err(ExecuteEndpointRequestError::Unauthorized(api_error)),
                    false,
                )
            }
            StatusCode::NOT_FOUND => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(ExecuteEndpointRequestError::NonUtf8Bytes)?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)
                    .map_err(|e| ExecuteEndpointRequestError::DeserializationError(e, body))?;
                (Err(ExecuteEndpointRequestError::NotFound(api_error)), true)
            }
            StatusCode::FORBIDDEN => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(ExecuteEndpointRequestError::NonUtf8Bytes)?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)
                    .map_err(|e| ExecuteEndpointRequestError::DeserializationError(e, body))?;
                (
                    Err(ExecuteEndpointRequestError::Forbidden(api_error)),
                    false,
                )
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(ExecuteEndpointRequestError::NonUtf8Bytes)?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)
                    .map_err(|e| ExecuteEndpointRequestError::DeserializationError(e, body))?;
                (
                    Err(ExecuteEndpointRequestError::InternalServerError(api_error)),
                    true,
                )
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(ExecuteEndpointRequestError::NonUtf8Bytes)?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer).ok();
                (
                    Err(ExecuteEndpointRequestError::RateLimited(api_error)),
                    true,
                )
            }
            _ => (
                Err(ExecuteEndpointRequestError::ResponseNotOk(Box::new(
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
                if current_try > max_retries {
                    tracing::debug!("No retries left, returning error.");
                    break Err(e);
                }
                tracing::debug!("API returned error, retrying: {e:?}");
            }
        }
    }
}
