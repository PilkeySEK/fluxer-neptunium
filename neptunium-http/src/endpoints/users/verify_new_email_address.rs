use bon::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct VerifyNewEmailAddress {
    #[builder(into)]
    pub ticket: String,
    #[builder(into)]
    pub code: String,
    #[builder(into)]
    pub original_proof: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct VerifyNewEmailAddressResponse {
    pub email_token: String,
}

impl Endpoint for VerifyNewEmailAddress {
    type Response = VerifyNewEmailAddressResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/email-change/verify-new".to_owned())
            .build()
    }
}
