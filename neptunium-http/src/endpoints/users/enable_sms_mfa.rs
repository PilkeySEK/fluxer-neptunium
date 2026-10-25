use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct EnableSmsMfa {
    pub auth: SudoVerification,
}

impl Endpoint for EnableSmsMfa {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.auth).unwrap())
            .path("/users/@me/mfa/sms/enable".to_owned())
            .build()
    }
}
