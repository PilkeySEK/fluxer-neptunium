use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DeleteCurrentUserAccount {
    pub auth: SudoVerification,
}

impl Endpoint for DeleteCurrentUserAccount {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.auth).unwrap())
            .path("/users/@me/delete".to_owned())
            .build()
    }
}
