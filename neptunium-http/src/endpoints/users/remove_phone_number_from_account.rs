use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct RemovePhoneNumberFromAccount {
    pub auth: SudoVerification,
}

impl Endpoint for RemovePhoneNumberFromAccount {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .body(serde_json::to_string(&self.auth).unwrap())
            .path("/users/@me/phone".to_owned())
            .build()
    }
}
