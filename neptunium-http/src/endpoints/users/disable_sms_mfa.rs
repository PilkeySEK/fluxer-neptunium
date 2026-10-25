use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DisableSmsMfa {
    pub auth: SudoVerification,
}

impl Endpoint for DisableSmsMfa {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.auth).unwrap())
            .path("/users/@me/mfa/sms/disable".to_owned())
            .build()
    }
}
