use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct GetWebauthnRegistrationOptions {
    pub auth: SudoVerification,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GetWebauthnRegistrationOptionsResponse {
    pub challenge: String,
}

impl Endpoint for GetWebauthnRegistrationOptions {
    type Response = GetWebauthnRegistrationOptionsResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.auth).unwrap())
            .path("/users/@me/mfa/webauthn/credentials/registration-options".to_owned())
            .build()
    }
}
