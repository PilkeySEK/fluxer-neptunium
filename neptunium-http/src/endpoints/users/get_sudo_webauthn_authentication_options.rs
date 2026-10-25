use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct GetSudoWebauthnAuthenticationOptions;

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct GetSudoWebauthnAuthenticationOptionsResponse {
    /// The WebAuthn challenge.
    #[expect(clippy::doc_markdown)]
    pub challenge: String,
}

impl Endpoint for GetSudoWebauthnAuthenticationOptions {
    type Response = GetSudoWebauthnAuthenticationOptionsResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path("/users/@me/sudo/webauthn/authentication-options".to_owned())
            .build()
    }
}
