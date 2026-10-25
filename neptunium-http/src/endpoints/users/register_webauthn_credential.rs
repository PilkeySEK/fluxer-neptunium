use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
#[expect(clippy::doc_markdown)]
pub struct RegisterWebauthnCredential {
    // TODO: What type is this??
    /// WebAuthn registration response.
    pub response: serde_json::Value,
    /// The challenge from registration options.
    #[builder(into)]
    pub challenge: String,
    /// User-assigned name for the credential.
    #[builder(into)]
    pub name: String,
    #[serde(flatten)]
    pub auth: SudoVerification,
}

impl Endpoint for RegisterWebauthnCredential {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/mfa/webauthn/credentials".to_owned())
            .build()
    }
}
