use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DeleteWebauthnCredential {
    #[builder(into)]
    pub credential_id: String,
    pub auth: SudoVerification,
}

impl Endpoint for DeleteWebauthnCredential {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .body(serde_json::to_string(&self.auth).unwrap())
            .path(format!(
                "/users/@me/mfa/webauthn/credentials/{}",
                self.credential_id
            ))
            .build()
    }
}
