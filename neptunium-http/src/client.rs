use reqwest::StatusCode;
use serde_json::Deserializer;

use crate::{
    DEFAULT_API_BASE_URL, DEFAULT_USER_AGENT, VERSION,
    endpoints::{Endpoint, ExecuteEndpointRequestError, ResponseBody},
};

/// Bot tokens have `Bot ` prefix, user tokens do not.
#[cfg(feature = "user_api")]
#[derive(Copy, Clone, Debug, Default)]
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
    pub(crate) token_type: TokenType,
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) user_agent: String,
}

impl HttpClient {
    #[must_use]
    pub fn new(token: String, #[cfg(feature = "user_api")] token_type: TokenType) -> Self {
        Self {
            api_base_url: DEFAULT_API_BASE_URL.to_owned(),
            reqwest_client: reqwest::Client::default(),
            token: zeroize::Zeroizing::new(token),
            #[cfg(feature = "user_api")]
            token_type,
            user_agent: format!("{DEFAULT_USER_AGENT}/{VERSION}"),
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
    pub async fn execute<T: Endpoint + Send>(
        &self,
        endpoint: T,
    ) -> Result<T::Response, Box<ExecuteEndpointRequestError>> {
        let request = endpoint.into_request();
        let response = request.execute(self).await?;
        tracing::trace!("API response: {:?}", response);

        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => {
                let body = response.bytes().await?.to_vec();
                tracing::trace!("API response body: {:?}", body);
                ResponseBody::deserialize(body)
            }
            StatusCode::BAD_REQUEST => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                Err(Box::new(ExecuteEndpointRequestError::BadRequest(api_error)))
            }
            StatusCode::UNAUTHORIZED => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                Err(Box::new(ExecuteEndpointRequestError::Unauthorized(
                    api_error,
                )))
            }
            StatusCode::NOT_FOUND => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                Err(Box::new(ExecuteEndpointRequestError::NotFound(api_error)))
            }
            StatusCode::FORBIDDEN => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                Err(Box::new(ExecuteEndpointRequestError::Forbidden(api_error)))
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                Err(Box::new(ExecuteEndpointRequestError::InternalServerError(
                    api_error,
                )))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let body = String::from_utf8(response.bytes().await?.to_vec())
                    .map_err(|e| Box::new(ExecuteEndpointRequestError::NonUtf8Bytes(e)))?;
                let mut deserializer = Deserializer::from_str(&body);
                let api_error = serde_path_to_error::deserialize(&mut deserializer)?;
                Err(Box::new(ExecuteEndpointRequestError::RateLimited(
                    api_error,
                )))
            }
            _ => Err(Box::new(ExecuteEndpointRequestError::ResponseNotOk(
                response,
            ))),
        }
    }
}
