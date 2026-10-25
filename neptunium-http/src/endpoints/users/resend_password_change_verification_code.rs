use bon::Builder;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct ResendPasswordChangeVerificationCode {
    #[builder(into)]
    pub ticket: String,
}

impl Endpoint for ResendPasswordChangeVerificationCode {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/password-change/resend".to_owned())
            .build()
    }
}
