use bon::Builder;
use neptunium_model::{
    id::{Id, marker::UserMarker},
    user::PartialUser,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegisterThemeType {
    Dark,
    /// Original neutral gray palette.
    DarkLegacy,
    Coal,
    Light,
    System,
}

#[derive(Serialize, Deserialize, Builder, Clone, Debug)]
pub struct Register {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// If not provided, a random one will be generated. 1-32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Must be in the format `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Whether the user consents to the terms of service.
    #[builder(default = false)]
    pub consent: bool,
    /// The invite code for the guild to join after registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,
    /// Admin-issued registration URL code to use for this registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_url_code: Option<String>,
    /// Initial UI theme preference for the new account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<RegisterThemeType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenWithUserIdResponse {
    pub token: String,
    pub user_id: Id<UserMarker>,
    pub user: PartialUser,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MfaMethod {
    Totp,
    WebAuthn,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MfaRequiredResponse {
    pub mfa: serde_bool::True,
    /// MFA ticket to use when completing MFA verification.
    pub ticket: String,
    /// List of allowed MFA methods.
    pub allowed_methods: Vec<MfaMethod>,
    /// Whether TOTP authenticator MFA is available.
    pub totp: bool,
    /// Whether WebAuthn security key MFA is available.
    pub webauthn: bool,
}

/// Registration succeeded and is waiting for admin approval.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct RegistrationPendingApprovalResponse {
    pub registration_pending_approval: serde_bool::True,
    pub user_id: Id<UserMarker>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum RegisterResponse {
    TokenWithUserId(TokenWithUserIdResponse),
    MfaRequired(MfaRequiredResponse),
    RegistrationPendingApproval(RegistrationPendingApprovalResponse),
}

impl Endpoint for Register {
    type Response = RegisterResponse;
    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path("/auth/register".to_owned())
            .body(serde_json::to_string(&self).unwrap())
            .build()
    }
}
