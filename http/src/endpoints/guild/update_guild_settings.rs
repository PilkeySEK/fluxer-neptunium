use bon::Builder;
use neptunium_model::{
    guild::{
        GuildResponse,
        properties::{
            DefaultMessageNotifications, GuildExplicitContentFilter, GuildFeatureFlag,
            GuildMfaLevel, GuildVerificationLevel, NsfwLevel, SplashCardAlignment,
            SystemChannelFlags,
        },
    },
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
    misc::ImageHash,
    time::timestamp::{Timestamp, representations::Iso8601},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct UpdateGuildSettingsBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Hash of the guild icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<SystemChannelFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<DefaultMessageNotifications>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<GuildVerificationLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_level: Option<GuildMfaLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw_level: Option<NsfwLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<GuildExplicitContentFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    /// Base64-encoded image data for the guild splash screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash: Option<String>,
    /// Base64-encoded image data for the embedded invite splash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_splash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash_card_alignment: Option<SplashCardAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<GuildFeatureFlag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_history_cutoff: Option<Timestamp<Iso8601>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub banner_height: Option<u32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub banner_width: Option<u32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub disabled_operations: Option<GuildOperations>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub embed_splash_height: Option<i32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub embed_splash_width: Option<i32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub owner_id: Option<Id<UserMarker>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub permissions: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub rules_channel_id: Option<Id<ChannelMarker>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub splash_height: Option<i32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub splash_width: Option<i32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub vanity_url_code: Option<String>,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateGuildSettings {
    pub guild_id: Id<GuildMarker>,
    pub body: UpdateGuildSettingsBody,
}

impl Endpoint for UpdateGuildSettings {
    type Response = GuildResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}", self.guild_id))
            .build()
    }
}
