use std::collections::HashMap;

use bon::Builder;
use reqwest::{
    Error, Method, Response,
    header::{HeaderMap, HeaderValue},
};

use crate::client::HttpClient;

#[derive(Clone, Debug, Builder)]
pub struct Request {
    pub body: Option<String>,
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub method: Method,
    pub path: String,
    #[builder(default = true)]
    pub use_authorization_token: bool,
    pub params: Option<HashMap<String, String>>,
}

impl Request {
    /// Execute the request. For a non-consuming version, see [`Self::borrowed_execute`].
    /// # Errors
    /// Returns an error if there was an error while sending the request, a redirect loop was detected or the redirect limit was exhausted.
    pub async fn execute(self, client: &HttpClient) -> Result<Response, Error> {
        let mut request = client
            .reqwest_client
            .request(self.method, format!("{}{}", client.api_base_url, self.path))
            .header("User-Agent", &client.user_agent);
        if let Some(headers) = self.headers {
            request = request.headers(headers);
        }
        if self.use_authorization_token {
            request = request.header("Authorization", format!("Bot {}", *client.token));
        }
        if let Some(body) = self.body {
            request = request.body(body);
        }
        if let Some(params) = self.params {
            request = request.query(&params);
        }

        request.send().await
    }

    /// Execute the request without consuming it. This clones certain fields when necessary. For a consuming version, see [`Self::execute`].
    /// # Errors
    /// Returns the same errors as [`Self::execute`].
    pub async fn borrowed_execute(&self, client: &HttpClient) -> Result<Response, Error> {
        let mut request = client
            .reqwest_client
            .request(
                self.method.clone(),
                format!("{}{}", client.api_base_url, self.path),
            )
            .header("User-Agent", &client.user_agent);
        if let Some(headers) = &self.headers {
            request = request.headers(headers.clone());
        }
        if self.use_authorization_token {
            request = request.header("Authorization", format!("Bot {}", *client.token));
        }
        if let Some(body) = &self.body {
            request = request.body(body.clone());
        }

        request.send().await
    }
}
