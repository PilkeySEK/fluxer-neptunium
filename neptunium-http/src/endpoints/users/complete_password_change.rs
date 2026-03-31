use bon::Builder;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct CompletePasswordChange {
    #[builder(into)]
    pub ticket: String,
    #[builder(into)]
    pub verification_proof: String,
    /// 8-256 characters.
    #[builder(into)]
    pub new_password: String,
}

impl Endpoint for CompletePasswordChange {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/password-change/complete".to_owned())
            .build()
    }
}
