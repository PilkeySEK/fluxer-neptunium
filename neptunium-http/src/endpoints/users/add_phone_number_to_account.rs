use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct AddPhoneNumberToAccount {
    #[builder(into)]
    pub phone_token: String,
    #[serde(flatten)]
    pub auth: SudoVerification,
}

impl Endpoint for AddPhoneNumberToAccount {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/phone".to_owned())
            .build()
    }
}
