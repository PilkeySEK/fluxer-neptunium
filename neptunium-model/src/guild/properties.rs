//! Various properties of a guild

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    id::{
        Id,
        marker::{EmojiMarker, StickerMarker},
    },
    serde_bitflags,
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
#[derive(Deserialize, Clone, Debug)]
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

/// Required MFA level for moderation actions
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
    /// Serializes/Deserializes as a string.
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
    /// Guild can have an animated banner.
    AnimatedBanner,
    /// Guild can have a banner.
    Banner,
    /// Guild banner is detached from splash.
    DetachedBanner,
    // https://github.com/fluxerapp/fluxer/blob/03813bbe17db008452f0f1be3090a7d2970a5447/packages/constants/src/GuildConstants.tsx#L115
    Discoverable,
    /// Guild can have an invite splash.
    InviteSplash,
    /// Guild has invites disabled.
    InvitesDisabled,
    /// Guild allows flexible text channel names.
    TextChannelFlexibleNames,
    /// Guild has increased emoji slots.
    MoreEmoji,
    /// Guild has increased sticker slots.
    MoreStickers,
    /// Guild has unlimited emoji slots.
    UnlimitedEmoji,
    /// Guild has unlimited sticker slots.
    UnlimitedStickers,
    /// Guild allows purging expressions.
    ExpressionPurgeAllowed,
    /// Guild can have a vanity URL.
    VanityUrl,
    /// Guild is verified.
    Verified,
    /// Guild has VIP voice features.
    VipVoice,
    /// Guild is unavailable for everyone.
    UnavailableForEveryone,
    /// Guild is unavailable except for staff.
    UnavailableForEveryoneButStaff,
    /// Guild is a visionary guild.
    Visionary,
    /// Guild is an operator guild.
    Operator,
    /// Guild has large guild overrides enabled.
    LargeGuildOverride,
    /// Guild has increased member capacity enabled.
    VeryLargeGuild,
    /// Guild has managed message scheduling.
    MtMessageScheduling,
    /// Guild has managed expression packs.
    MtExpressionPacks,
    /// A raid has been detected in this guild.
    RaidDetected,
}
