use bon::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CheckUsernameTagAvailabilityParams {
    #[builder(into)]
    pub username: String,
    #[builder(into)]
    pub discriminator: String,
}

#[derive(Builder, Clone, Debug)]
pub struct CheckUsernameTagAvailability {
    pub params: CheckUsernameTagAvailabilityParams,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct CheckUsernameTagAvailabilityResponse {
    pub taken: bool,
}

impl Endpoint for CheckUsernameTagAvailability {
    type Response = CheckUsernameTagAvailabilityResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .params(serde_qs::to_string(&self.params).unwrap())
            .path("/users/check-tag".to_owned())
            .build()
    }
}
