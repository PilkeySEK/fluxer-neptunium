use neptunium_model::user::auth::handoff::HandoffCode;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetHandoffInfo {
    pub code: HandoffCode,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HandoffInfoStatus {
    Pending,
    Expired,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AuthSessionLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HandoffInfoClientInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<AuthSessionLocation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HandoffInfo {
    pub status: HandoffInfoStatus,
    pub client_info: HandoffInfoClientInfo,
}

impl Endpoint for GetHandoffInfo {
    type Response = HandoffInfo;
    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/auth/handoff/{}/info", self.code))
            .build()
    }
}
