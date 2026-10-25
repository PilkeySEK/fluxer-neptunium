use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Builder, Clone, Debug)]
pub struct DisableTotpMfa {
    /// The TOTP code.
    #[builder(into)]
    pub code: String,
    #[serde(flatten)]
    pub auth: SudoVerification,
}

impl Endpoint for DisableTotpMfa {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/mfa/totp/disable".to_owned())
            .build()
    }
}
