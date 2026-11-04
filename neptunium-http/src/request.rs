use std::collections::HashMap;

use bon::Builder;
use reqwest::{
    Error, Method, Response,
    header::{HeaderMap, HeaderValue},
};

use crate::client::TokenType;

#[derive(Clone, Debug, Builder)]
pub struct Request {
    pub body: Option<String>,
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub method: Method,
    pub path: String,
    #[builder(default = true)]
    pub use_authorization_token: bool,
    pub params: Option<HashMap<String, String>>,
    pub audit_log_reason: Option<String>,
}

pub struct RequestExecuteClientInfo<'a> {
    pub reqwest_client: &'a reqwest::Client,
    pub base_url: &'a str,
    pub token_info: Option<(&'a str, TokenType)>,
    pub user_agent: Option<&'a str>,
}

impl Request {
    /// Execute the request. For a non-consuming version, see [`Self::borrowed_execute`].
    /// # Errors
    /// Returns an error if there was an error while sending the request, a redirect loop was detected or the redirect limit was exhausted.
    pub async fn execute(
        self,
        client_info: RequestExecuteClientInfo<'_>,
    ) -> Result<Response, Error> {
        let mut request = client_info.reqwest_client.request(
            self.method,
            format!("{}{}", client_info.base_url, self.path),
        );
        if let Some(user_agent) = client_info.user_agent {
            request = request.header("User-Agent", user_agent);
        }
        if let Some(headers) = self.headers {
            request = request.headers(headers);
        }
        if self.use_authorization_token
            && let Some((token, token_type)) = client_info.token_info
        {
            let token = match token_type {
                TokenType::Bot => format!("Bot {}", token),
                TokenType::User => token.to_owned(),
                TokenType::Bearer => format!("Bearer {}", token),
            };

            request = request.header("Authorization", token);
        }
        if let Some(body) = self.body {
            request = request.body(body);
        }
        if let Some(audit_log_reason) = self.audit_log_reason {
            request = request.header("X-Audit-Log-Reason", audit_log_reason);
        }
        if let Some(params) = self.params
            && !params.is_empty()
        {
            request = request.query(&params);
        }

        request.send().await
    }
}
