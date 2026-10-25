use bon::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct VerifyOriginalEmailAddress {
    #[builder(into)]
    pub ticket: String,
    #[builder(into)]
    pub code: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct VerifyOriginalEmailAddressResponse {
    pub oringinal_proof: String,
}

impl Endpoint for VerifyOriginalEmailAddress {
    type Response = VerifyOriginalEmailAddressResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/email-change/verify-original".to_owned())
            .build()
    }
}
