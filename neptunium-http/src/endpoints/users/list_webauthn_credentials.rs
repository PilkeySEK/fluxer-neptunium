use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct ListWebauthnCredentials;

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ListWebauthnCredentialsResponseEntry {
    pub id: String,
    pub name: String,
    // ISO8601
    pub created_at: jiff::Timestamp,
    // ISO8601
    pub last_used_at: Option<jiff::Timestamp>,
}

impl Endpoint for ListWebauthnCredentials {
    type Response = Vec<ListWebauthnCredentialsResponseEntry>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/mfa/webauthn/credentials".to_owned())
            .build()
    }
}
