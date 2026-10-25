use bon::Builder;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct ResendNewEmailConfirmation {
    #[builder(into)]
    pub ticket: String,
}

impl Endpoint for ResendNewEmailConfirmation {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/email-change/resend-new".to_owned())
            .build()
    }
}
