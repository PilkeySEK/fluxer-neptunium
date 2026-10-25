//! Various properties of a guild.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    id::{
        Id,
        marker::{EmojiMarker, StickerMarker},
    },
    misc::serde_bitflags,
    user::PartialUser,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildEmoji {
    pub id: Id<EmojiMarker>,
    pub name: String,
    pub animated: bool,
}

/*
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildSticker {
    pub guild_id: Id<GuildMarker>,
    pub id: Id<StickerMarker>,
    pub name: String,
    pub description: Option<String>,
    pub format_type: i32,
    pub tags: Option<Vec<String>>,
    pub creator_id: Id<UserMarker>,
}
*/

// GuildStickerResponse
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildSticker {
    pub id: Id<StickerMarker>,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub animated: bool,
}

// GuildStickerWithUserResponse
#[derive(Deserialize, Clone, Debug)]
pub struct GuildStickerWithUser {
    pub id: Id<StickerMarker>,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub animated: bool,
    pub user: PartialUser,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum DefaultMessageNotifications {
    AllMessages = 0,
    MentionsOnly = 1,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum GuildExplicitContentFilter {
    None = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

/// Required MFA level for moderation actions.
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum GuildMfaLevel {
    NoMfaRequirement = 0,
    RequiresMfa = 1,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum NsfwLevel {
    Default = 0,
    HasExplicitContent = 1,
    IsSafe = 2,
    IsAgeRestricted = 3,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum SplashCardAlignment {
    Center = 0,
    Left = 1,
    Right = 2,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum GuildVerificationLevel {
    Unrestricted = 0,
    VerifiedEmail = 1,
    RegisteredForMoreThan5Minutes = 2,
    MemberOfServerForMoreThan10Minutes = 3,
    VerifiedPhoneNumber = 4,
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct GuildOperations: u32 {
        const PUSH_NOTIFICATIONS = 1 << 0;
        const EVERYONE_MENTIONS = 1 << 1;
        const TYPING_EVENTS = 1 << 2;
        const INSTANT_INVITES = 1 << 3;
        const SEND_MESSAGE = 1 << 4;
        const REACTIONS = 1 << 5;
        const MEMBER_LIST_UPDATES = 1 << 6;
    }
}

serde_bitflags! {GuildOperations, u32}
/* serde_bitflags! {GuildOperations, String} */

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct SystemChannelFlags: u32 {
        const SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;
    }
}

serde_bitflags! {SystemChannelFlags, u32}

/// A guild feature flag.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GuildFeatureFlag {
    AnimatedIcon,
    /// Publishing from and following the guild’s announcement channels is disabled.
    /// Set by an instance administrator.
    AnnouncementChannelsDisabled,
    /// Guild can have an animated banner.
    AnimatedBanner,
    #[serde(rename = "AUDIO_BITRATE_128_KBPS")]
    AudioBitrate128Kbps,
    #[serde(rename = "AUDIO_BITRATE_256_KBPS")]
    AudioBitrate256Kbps,
    #[serde(rename = "AUDIO_BITRATE_384_KBPS")]
    AudioBitrate384Kbps,
    /// Guild can use a banner.
    Banner,
    /// Stickers cannot be cloned using the built-in feature.
    #[deprecated = "Use `CloneEmojiEnabled`"]
    CloneEmojiDisabled,
    /// Stickers can be cloned using the built-in feature.
    CloneEmojiEnabled,
    /// Stickers cannot be cloned using the built-in feature.
    #[deprecated = "Use `CloneStickerEnabled`"]
    CloneStickerDisabled,
    /// Stickers can be cloned using the built-in feature.
    CloneStickerEnabled,
    /// Guild banner is detached from splash.
    DetachedBanner,
    /// Guild can have an invite splash.
    InviteSplash,
    /// Guild has invites disabled.
    InvitesDisabled,
    /// Raid detection is active and invites are restricted.
    RaidDetected,
    /// Guild allows flexible text channel names.
    TextChannelFlexibleNames,
    /// The owner crown is hidden in the UI.
    HideOwnerCrown,
    /// (Legacy) Guild has increased emoji slots.
    MoreEmoji,
    /// (Legacy) Guild has increased sticker slots.
    MoreStickers,
    /// Guild has effectively (999999) unlimited emoji slots.
    UnlimitedEmoji,
    /// Guild has effectively (999999) unlimited sticker slots.
    UnlimitedStickers,
    /// Expression assets can be purged through delete operations.
    ExpressionPurgeAllowed,
    /// Guild can have a vanity URL.
    VanityUrl,
    /// Guild is present in public discovery.
    Discoverable,
    /// Guild has partnered status.
    Partnered,
    /// Guild is verified.
    Verified,
    /// Guild can use voice regions that are restricted to VIP guilds.
    VipVoice,
    /// The guild has end-to-end encrypted voice chats.
    #[serde(rename = "VOICE_E2EE")]
    VoiceE2EE,
    /// Guild is unavailable for everyone.
    UnavailableForEveryone,
    /// Guild is unavailable except for staff.
    UnavailableForEveryoneButStaff,
    /// While the guild is unavailable, the Gateway sends its unavailable guild entry
    /// with `unavailable_hidden: true`.
    UnavailableHidden,
    /// Guild is a visionary guild.
    Visionary,
    /// Guild is marked as a large guild.
    LargeGuildOverride,
    /// Guild member capacity is raised.
    VeryLargeGuild,
    #[serde(untagged)]
    Other(String),
}
