use neptunium_model::time::timestamp::{Timestamp, representations::Iso8601};
use reqwest::Method;
use serde::Deserialize;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct StartPasswordChange;

#[derive(Deserialize, Clone, Debug)]
pub struct StartPasswordChangeResponse {
    pub ticket: String,
    pub code_expires_at: Timestamp<Iso8601>,
    pub resend_available_at: Option<Timestamp<Iso8601>>,
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
