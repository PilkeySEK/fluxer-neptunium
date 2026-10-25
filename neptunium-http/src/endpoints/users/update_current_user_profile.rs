use bon::Builder;
use neptunium_model::gateway::payload::incoming::UserPrivateResponse;
#[cfg(feature = "user_api")]
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct UpdateCurrentUserProfile {
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    /// Set this to `Some(None)` to remove the global name.
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub auth: Option<SudoVerification>,
    /// Base64-encoded avatar image. Set this to `Some(None)` to remove the avatar.
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
    /// Base64-encoded banner image. Set this to `Some(None)` to remove the banner.
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Option<String>>,
    /// Maximum 320 characters. Set this to `Some(None)` to remove the bio.
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<Option<String>>,
    /// Maximum 40 characters. Set this to `Some(None)` to remove the pronouns.
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_badge_hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_badge_masked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_badge_timestamp_hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_badge_sequence_hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_enabled_override: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_dismissed_premium_onboarding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_unread_gift_inventory: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_mobile_client: Option<bool>,
    /// Email change token for updating email.
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_token: Option<String>,
}

impl Endpoint for UpdateCurrentUserProfile {
    type Response = UserPrivateResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me".to_owned())
            .build()
    }
}
