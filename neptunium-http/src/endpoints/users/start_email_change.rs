use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct StartEmailChange;

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct StartEmailChangeResponse {
    pub ticket: String,
    pub require_original: bool,
    pub original_email: Option<String>,
    pub original_proof: Option<String>,
    // ISO8601
    pub original_code_expires_at: Option<jiff::Timestamp>,
    // ISO8601
    pub resend_available_at: Option<jiff::Timestamp>,
}

impl Endpoint for StartEmailChange {
    type Response = StartEmailChangeResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(json!({}).to_string())
            .path("/users/@me/email-change/start".to_owned())
            .build()
    }
}
