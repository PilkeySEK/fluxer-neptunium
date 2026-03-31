use bon::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct VerifyPasswordChangeCode {
    #[builder(into)]
    pub ticket: String,
    #[builder(into)]
    pub code: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VerifyPasswordChangeCodeResponse {
    pub verification_proof: String,
}

impl Endpoint for VerifyPasswordChangeCode {
    type Response = VerifyPasswordChangeCodeResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/password-change/verify".to_owned())
            .build()
    }
}
