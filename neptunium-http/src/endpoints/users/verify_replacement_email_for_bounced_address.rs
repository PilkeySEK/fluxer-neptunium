use bon::Builder;
use neptunium_model::gateway::payload::incoming::UserPrivateResponse;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct VerifyReplacementEmailForBouncedAddress {
    #[builder(into)]
    pub ticket: String,
    #[builder(into)]
    pub code: String,
}

impl Endpoint for VerifyReplacementEmailForBouncedAddress {
    type Response = UserPrivateResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/email-change/bounced/verify-new".to_owned())
            .build()
    }
}
