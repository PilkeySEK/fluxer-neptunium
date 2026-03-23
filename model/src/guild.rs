use serde::{Deserialize, Serialize};

use crate::{
    guild::{
        permissions::GuildRole,
        properties::{
            DefaultMessageNotifications, GuildEmoji, GuildExplicitContentFilter, GuildFeatureFlag,
            GuildMfaLevel, GuildOperations, GuildSticker, GuildVerificationLevel, NsfwLevel,
            SplashCardAlignment, SystemChannelFlags,
        },
    },
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
    misc::ImageHash,
    time::timestamp::{Timestamp, representations::Iso8601},
};

pub mod audit_log;
pub mod bans;
pub mod default_message_notification_level;
pub mod explicit_content_filter;
pub mod member;
pub mod permissions;
pub mod properties;
pub mod webhook;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildProperties {
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: u32,
    pub banner: Option<String>,
    pub banner_height: Option<u32>,
    pub banner_width: Option<u32>,
    pub default_message_notifications: DefaultMessageNotifications,
    pub disabled_operations: GuildOperations,
    /// Base64-encoded image data for the embedded invite splash.
    pub embed_splash: Option<String>,
    pub embed_splash_height: Option<i32>,
    pub embed_splash_width: Option<i32>,
    pub explicit_content_filter: GuildExplicitContentFilter,
    pub features: Vec<GuildFeatureFlag>,
    /// Hash of the guild icon
    pub icon: Option<ImageHash>,
    pub id: Id<GuildMarker>,
    pub message_history_cutoff: Option<Timestamp<Iso8601>>,
    pub mfa_level: GuildMfaLevel,
    pub name: String,
    pub nsfw_level: NsfwLevel,
    pub owner_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    /// Base64-encoded image data for the guild splash screen.
    pub splash: Option<String>,
    pub splash_card_alignment: SplashCardAlignment,
    pub splash_height: Option<i32>,
    pub splash_width: Option<i32>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub vanity_url_code: Option<String>,
    pub verification_level: GuildVerificationLevel,
}

// TODO: Find out what the difference between GuildResponse and Guild is, it's weird idk

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildResponse {
    #[serde(flatten)]
    pub properties: GuildProperties,
    pub guild_id: Id<GuildMarker>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Guild {
    pub properties: GuildProperties,
    pub emojis: Vec<GuildEmoji>,
    pub stickers: Vec<GuildSticker>,
    pub roles: Vec<GuildRole>,
    pub online_count: i32,
    pub id: Id<GuildMarker>,
    pub member_count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildPartial {
    pub id: Id<GuildMarker>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash_height: Option<u32>,
    pub splash_card_alignment: SplashCardAlignment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_splash: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_splash_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_splash_height: Option<u32>,
    pub features: Vec<GuildFeatureFlag>,
}
