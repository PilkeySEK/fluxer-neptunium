use bon::Builder;
use neptunium_model::time::timestamp::{Timestamp, representations::Iso8601};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct RequestNewEmailAddress {
    #[builder(into)]
    pub ticket: String,
    #[builder(into)]
    pub new_email: String,
    /// Proof token obtained from verifying the original email.
    #[builder(into)]
    pub original_proof: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct RequestNewEmailAddressResponse {
    pub ticket: String,
    pub new_email: String,
    pub new_code_expires_at: Timestamp<Iso8601>,
    pub resend_available_at: Option<Timestamp<Iso8601>>,
}

impl Endpoint for RequestNewEmailAddress {
    type Response = RequestNewEmailAddressResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/email-change/request-new".to_owned())
            .build()
    }
}
