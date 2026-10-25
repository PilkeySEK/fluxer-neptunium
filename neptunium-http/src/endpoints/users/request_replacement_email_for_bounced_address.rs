use bon::Builder;
use reqwest::Method;
use serde::Serialize;

use crate::{
    endpoints::{Endpoint, users::RequestNewEmailAddressResponse},
    request::Request,
};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct RequestReplacementEmailForBouncedAddress {
    #[builder(into)]
    pub new_email: String,
}

impl Endpoint for RequestReplacementEmailForBouncedAddress {
    type Response = RequestNewEmailAddressResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/email-change/bounced/request-new".to_owned())
            .build()
    }
}
