use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct StartPasswordChange;

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct StartPasswordChangeResponse {
    pub ticket: String,
    // ISO8601
    pub code_expires_at: jiff::Timestamp,
    // ISO8601
    pub resend_available_at: Option<jiff::Timestamp>,
}

impl Endpoint for StartPasswordChange {
    type Response = StartPasswordChangeResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(json!({}).to_string())
            .path("/users/@me/password-change/start".to_owned())
            .build()
    }
}
