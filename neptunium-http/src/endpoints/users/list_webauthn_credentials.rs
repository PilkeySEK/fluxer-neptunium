use neptunium_model::time::timestamp::{Timestamp, representations::Iso8601};
use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct ListWebauthnCredentials;

#[derive(Deserialize, Clone, Debug)]
pub struct ListWebauthnCredentialsResponseEntry {
    pub id: String,
    pub name: String,
    pub created_at: Timestamp<Iso8601>,
    pub last_used_at: Option<Timestamp<Iso8601>>,
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
