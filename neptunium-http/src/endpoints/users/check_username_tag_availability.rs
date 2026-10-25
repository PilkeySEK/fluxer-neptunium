use std::collections::HashMap;

use bon::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct CheckUsernameTagAvailability {
    #[builder(into)]
    pub username: String,
    #[builder(into)]
    pub discriminator: String,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct CheckUsernameTagAvailabilityResponse {
    pub taken: bool,
}

impl Endpoint for CheckUsernameTagAvailability {
    type Response = CheckUsernameTagAvailabilityResponse;

    fn into_request(self) -> crate::request::Request {
        let mut params = HashMap::new();
        params.insert("username".to_owned(), self.username);
        params.insert("discriminator".to_owned(), self.discriminator);

        Request::builder()
            .method(Method::GET)
            .params(params)
            .path("/users/check-tag".to_owned())
            .build()
    }
}
