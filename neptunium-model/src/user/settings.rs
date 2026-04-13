use std::collections::HashMap;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    gateway::presence::{CustomStatus, PresenceStatus},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
    misc::HexColor32,
    serde_bitflags,
    time::{
        duration::{Duration, representation::Seconds},
        timestamp::{Timestamp, representations::Iso8601},
    },
};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Locale {
    /// Arabic
    #[serde(rename = "ar")]
    Ar,
    /// Bulgarian
    #[serde(rename = "bg")]
    Bg,
    /// Czech
    #[serde(rename = "cs")]
    Cs,
    /// Danish
    #[serde(rename = "da")]
    Da,
    /// German
    #[serde(rename = "de")]
    De,
    /// Greek
    #[serde(rename = "el")]
    El,
    /// English (United Kingdom)
    #[serde(rename = "en-GB")]
    EnGb,
    /// English (United States)
    #[serde(rename = "en-US")]
    EnUs,
    /// Spanish (Spain)
    #[serde(rename = "es-ES")]
    EsEs,
    /// Spanish (Latin America)
    #[serde(rename = "es-419")]
    Es419,
    /// Finnish
    #[serde(rename = "fi")]
    Fi,
    /// French
    #[serde(rename = "fr")]
    Fr,
    /// Hebrew
    #[serde(rename = "he")]
    He,
    /// Hindi
    #[serde(rename = "hi")]
    Hi,
    /// Croatian
    #[serde(rename = "hr")]
    Hr,
    /// Hungarian
    #[serde(rename = "hu")]
    Hu,
    /// Indonesian
    #[serde(rename = "id")]
    Id,
    /// Italian
    #[serde(rename = "it")]
    It,
    /// Japanese
    #[serde(rename = "ja")]
    Ja,
    /// Korean
    #[serde(rename = "ko")]
    Ko,
    /// Lithuanian
    #[serde(rename = "lt")]
    Lt,
    /// Dutch
    #[serde(rename = "nl")]
    Nl,
    /// Norwegian
    #[serde(rename = "no")]
    No,
    /// Polish
    #[serde(rename = "pl")]
    Pl,
    /// Portuguese (Brazil)
    #[serde(rename = "pt-BR")]
    PtBr,
    /// Romanian
    #[serde(rename = "ro")]
    Ro,
    /// Russian
    #[serde(rename = "ru")]
    Ru,
    /// Swedish
    #[serde(rename = "sv-SE")]
    SvSe,
    /// Thai
    #[serde(rename = "th")]
    Th,
    /// Turkish
    #[serde(rename = "tr")]
    Tr,
    /// Ukrainian
    #[serde(rename = "uk")]
    Uk,
    /// Vietnamese
    #[serde(rename = "vi")]
    Vi,
    /// Chinese (Simplified)
    #[serde(rename = "zh-CN")]
    ZhCn,
    /// Chinese (Traditional)
    #[serde(rename = "zh-TW")]
    ZhTw,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum StickerAnimationOptions {
    AlwaysAnimate = 0,
    AnimateOnHoverOrInteraction = 1,
    NeverAnimate = 2,
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct FriendSourceFlags: u32 {
        /// Allow friend requests from users who share mutual friends.
        const MUTUAL_FRIENDS = 1 << 0;
        /// Allow friend requests from users in mutual guilds.
        const MUTUAL_GUILDS = 1 << 1;
        /// Allow friend requests from users with no existing relation.
        const NO_RELATION = 1 << 2;
    }
}

serde_bitflags! {FriendSourceFlags, u32}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct GroupDmAddPermissionFlags: u32 {
        /// Allow friends of friends to add user to group DMs.
        const FRIENDS_OF_FRIENDS = 1 << 0;
        /// Allow guild members to add user to group DMs.
        const GUILD_MEMBERS = 1 << 1;
        /// Allow everyone to add user to group DMs.
        const EVERYONE = 1 << 2;
        /// Allow only friends to add user to group DMs.
        const FRIENDS_ONLY = 1 << 3;
        /// Block everyone from adding user to group DMs.
        const NOBODY = 1 << 4;
    }
}

serde_bitflags! {GroupDmAddPermissionFlags, u32}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct GuildFolderFlags: u32 {
        /// Show the selected icon instead of guild previews when the folder is collapsed.
        const SHOW_ICON_WHEN_COLLAPSED = 1 << 0;
    }
}

serde_bitflags! {GuildFolderFlags, u32}

// In the Fluxer source, this is called `GuildFolderIconSchema`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GuildFoldersItemIcon {
    #[default]
    Folder,
    Star,
    Heart,
    Bookmark,
    GameController,
    Shield,
    MusicNote,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildFoldersItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<HexColor32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<GuildFolderFlags>,
    pub guild_ids: Option<Vec<Id<GuildMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<GuildFoldersItemIcon>,
    /// The unique identifier for the folder (-1 means uncategorized)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct IncomingCallFlags: u32 {
        /// Allow incoming calls from friends of friends.
        const FRIENDS_OF_FRIENDS = 1 << 0;
        /// Allow incoming calls from guild members.
        const GUILD_MEMBERS = 1 << 1;
        /// Allow incoming calls from everyone.
        const EVERYONE = 1 << 2;
        /// Allow incoming calls only from friends.
        const FRIENDS_ONLY = 1 << 3;
        /// Block all incoming calls.
        const NOBODY = 1 << 4;
        /// Allow calls from everyone but receive them silently.
        const SILENT_EVERYONE = 1 << 5;
    }
}

serde_bitflags! {IncomingCallFlags, u32}

/// Spoiler rendering preference.
#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RenderSpoilers {
    AlwaysReveal = 0,
    RevealOnClick = 1,
    RevealIfModerator = 2,
}

/// Time format preference.
#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TimeFormatTypes {
    /// Automatically detect time format based on locale.
    AutoDetect = 0,
    /// Use 12-hour time format (AM/PM).
    TwelveHour = 1,
    /// Use 24-hour time format.
    TwentyFourHour = 2,
}

// TODO: Rename this struct and others to be more consistently named after "saved media"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteMeme {
    /// Unique identifier for the favorite meme
    pub id: String,
    /// ID of the user who owns this favorite meme
    pub user_id: String,
    /// Display name of the meme
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    /// Tags for categorizing and searching the meme
    pub tags: Vec<String>,
    /// ID of the attachment storing the meme
    pub attachment_id: String,
    /// Original filename of the meme
    pub filename: String,
    /// MIME type of the meme file
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    /// File size in bytes
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// CDN URL to access the meme
    pub url: String,
    /// Whether the meme is a video converted from GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_gifv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klipy_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenor_slug_id: Option<String>,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSettings {
    pub afk_timeout: Duration<Seconds>,
    pub animate_emoji: bool,
    pub animate_stickers: StickerAnimationOptions,
    pub bot_default_guilds_restricted: bool,
    pub bot_restricted_guilds: Vec<Id<GuildMarker>>,
    pub custom_status: Option<CustomStatus>,
    pub default_guilds_restricted: bool,
    pub default_hide_muted_channels: bool,
    pub developer_mode: bool,
    pub friend_source_flags: FriendSourceFlags,
    pub gif_auto_play: bool,
    pub group_dm_add_permission_flags: GroupDmAddPermissionFlags,
    pub guild_folders: Vec<GuildFoldersItem>,
    pub incoming_call_flags: IncomingCallFlags,
    pub inline_attachment_media: bool,
    pub inline_embed_media: bool,
    pub locale: Locale,
    pub message_display_compact: bool,
    pub render_embeds: bool,
    pub render_reactions: bool,
    pub render_spoilers: RenderSpoilers,
    pub restricted_guilds: Vec<Id<GuildMarker>>,
    pub status: PresenceStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_resets_at: Option<Timestamp<Iso8601>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_resets_to: Option<PresenceStatus>,
    pub theme: String,
    pub time_format: TimeFormatTypes,
    pub trusted_domains: Option<Vec<String>>,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum UserNotificationSettings {
    AllMessages = 0,
    MentionsOnly = 1,
    None = 2,
    InheritFromParent = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGuildSettingsMuteConfig {
    pub end_time: Option<Timestamp<Iso8601>>,
    pub selected_time_window: crate::time::duration::Duration<Seconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGuildSettingsChannelOverride {
    pub collapsed: bool,
    pub message_notifications: UserNotificationSettings,
    pub muted: bool,
    pub mute_config: Option<UserGuildSettingsMuteConfig>,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGuildSettings {
    /// `None` if these are the settings for the user DMs.
    pub guild_id: Option<Id<GuildMarker>>,
    /// The default notification level for the guild
    pub message_notifications: UserNotificationSettings,
    /// Whether the guild is muted
    pub muted: bool,
    pub mute_config: Option<UserGuildSettingsMuteConfig>,
    /// Whether mobile push notifications are enabled
    pub mobile_push: bool,
    /// Whether @everyone mentions are suppressed
    pub suppress_everyone: bool,
    /// Whether role mentions are suppressed
    pub suppress_roles: bool,
    /// Whether muted channels are hidden in the sidebar
    pub hide_muted_channels: bool,
    pub channel_overrides: Option<HashMap<Id<ChannelMarker>, UserGuildSettingsChannelOverride>>,
    /// The version number of these settings for sync
    pub version: i32,
}
