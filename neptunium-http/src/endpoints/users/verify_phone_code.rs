use bon::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct VerifyPhoneCode {
    #[builder(into)]
    pub phone: String,
    #[builder(into)]
    pub code: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VerifyPhoneCodeResponse {
    pub phone_token: String,
}

impl Endpoint for VerifyPhoneCode {
    type Response = VerifyPhoneCodeResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/phone/verify".to_owned())
            .build()
    }
}
