use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;
use serde::Serialize;

use crate::{
    endpoints::{Endpoint, users::MfaBackupCodesResponse},
    request::Request,
};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct EnableTotpMfa {
    #[builder(into)]
    pub secret: String,
    #[builder(into)]
    pub code: String,
    #[serde(flatten)]
    pub auth: SudoVerification,
}

impl Endpoint for EnableTotpMfa {
    type Response = MfaBackupCodesResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/mfa/totp/enable".to_owned())
            .build()
    }
}
