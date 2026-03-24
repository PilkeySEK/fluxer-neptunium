use serde::Deserialize;

use crate::{
    guild::{
        permissions::Permissions,
        properties::{
            DefaultMessageNotifications, GuildExplicitContentFilter, GuildFeatureFlag,
            GuildMfaLevel, GuildOperations, GuildVerificationLevel, NsfwLevel, SplashCardAlignment,
            SystemChannelFlags,
        },
    },
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
    misc::ImageHash,
    time::{
        duration::{Duration, representation::Seconds},
        timestamp::{Timestamp, representations::Iso8601},
    },
};

pub mod audit_log;
pub mod bans;
pub mod default_message_notification_level;
pub mod explicit_content_filter;
pub mod member;
pub mod permissions;
pub mod properties;
pub mod webhook;

// Source: https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/schema/src/domains/guild/GuildResponseSchemas.tsx#L95
#[derive(Deserialize, Clone, Debug)]
pub struct Guild {
    pub id: Id<GuildMarker>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
    // TODO: Maybe make a separate `GuildBannerInformation` struct
    // so that the types are nicer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_height: Option<u32>,
    // TODO: Maybe a GuildSplashInformation struct here too.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash_height: Option<u32>,
    pub splash_card_alignment: SplashCardAlignment,
    // TODO: Nicer types here too maybe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_splash: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_splash_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_splash_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanity_url_code: Option<String>,
    pub owner_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub system_channel_flags: SystemChannelFlags,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: Duration<Seconds>,
    pub features: Vec<GuildFeatureFlag>,
    pub verification_level: GuildVerificationLevel,
    pub mfa_level: GuildMfaLevel,
    pub nsfw_level: NsfwLevel,
    pub explicit_content_filter: GuildExplicitContentFilter,
    pub default_message_notifications: DefaultMessageNotifications,
    pub disabled_operations: GuildOperations,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_history_cutoff: Timestamp<Iso8601>,
    /// The current user permissions in this guild.
    #[serde(skip_serializing_if = "Option::is_none", rename = "permissions")]
    pub current_user_permissions: Permissions,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PartialGuild {
    pub id: Id<GuildMarker>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
    // TODO: Maybe make a separate `GuildBannerInformation` struct
    // so that the types are nicer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_height: Option<u32>,
    // TODO: Maybe a GuildSplashInformation struct here too.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash_height: Option<u32>,
    pub splash_card_alignment: SplashCardAlignment,
    // TODO: Nicer types here too maybe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_splash: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_splash_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_splash_height: Option<u32>,
    pub features: Vec<GuildFeatureFlag>,
}
