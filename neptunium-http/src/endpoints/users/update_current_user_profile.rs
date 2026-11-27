use bon::Builder;
#[cfg(feature = "user_api")]
use neptunium_model::user::auth::SudoVerification;
use neptunium_model::{gateway::payload::incoming::UserPrivateResponse, misc::MissingOrNull};
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
    /// Set this to `Null` to remove the global name.
    #[builder(into)]
    #[serde(default, skip_serializing_if = "MissingOrNull::is_missing")]
    pub global_name: MissingOrNull<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub auth: Option<SudoVerification>,
    /// Base64-encoded avatar image. Set this to `Null` to remove the avatar.
    #[builder(into)]
    #[serde(default, skip_serializing_if = "MissingOrNull::is_missing")]
    pub avatar: MissingOrNull<String>,
    /// Base64-encoded banner image. Set this to `Null` to remove the banner.
    #[builder(into)]
    #[serde(default, skip_serializing_if = "MissingOrNull::is_missing")]
    pub banner: MissingOrNull<String>,
    /// Maximum 320 characters. Set this to `Null` to remove the bio.
    #[builder(into)]
    #[serde(default, skip_serializing_if = "MissingOrNull::is_missing")]
    pub bio: MissingOrNull<String>,
    /// Maximum 40 characters. Set this to `Null` to remove the pronouns.
    #[builder(into)]
    #[serde(default, skip_serializing_if = "MissingOrNull::is_missing")]
    pub pronouns: MissingOrNull<String>,
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
