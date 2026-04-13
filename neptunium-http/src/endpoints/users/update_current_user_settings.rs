use bon::Builder;
use neptunium_model::{
    gateway::presence::{CustomStatus, PresenceStatus},
    id::{Id, marker::GuildMarker},
    time::{
        duration::{Duration, representation::Seconds},
        timestamp::{Timestamp, representations::Iso8601},
    },
    user::settings::{
        FriendSourceFlags, GroupDmAddPermissionFlags, GuildFoldersItem, IncomingCallFlags,
        LanguageTag, RenderSpoilers, StickerAnimationOptions, TimeFormatTypes, UserSettings,
    },
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct UpdateUserSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<Duration<Seconds>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate_emoji: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate_stickers: Option<StickerAnimationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_default_guilds_restricted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_restricted_guilds: Option<Vec<Id<GuildMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_status: Option<Option<CustomStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_guilds_restricted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_hide_muted_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friend_source_flags: Option<FriendSourceFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_auto_play: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_dm_add_permission_flags: Option<GroupDmAddPermissionFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_folders: Option<Vec<GuildFoldersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_call_flags: Option<IncomingCallFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_attachment_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_embed_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<LanguageTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_display_compact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_embeds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_reactions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_spoilers: Option<RenderSpoilers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_guilds: Option<Vec<Id<GuildMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PresenceStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_resets_at: Option<Timestamp<Iso8601>>,
    /// Set to `Some(None)` to reset this setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_resets_to: Option<Option<PresenceStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<TimeFormatTypes>,
    pub trusted_domains: Option<Vec<String>>,
}

impl Endpoint for UpdateUserSettings {
    type Response = UserSettings;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/settings".to_owned())
            .build()
    }
}
