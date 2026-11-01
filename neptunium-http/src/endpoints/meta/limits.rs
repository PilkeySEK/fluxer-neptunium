use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LimitKey {
    /// Maximum avatar upload size in bytes, up to 104857600 (user)
    AvatarMaxSize,
    /// Maximum emoji upload size in bytes, up to 104857600 (guild)
    EmojiMaxSize,
    /// Animated avatar uploads (user)
    FeatureAnimatedAvatar,
    /// Animated banner uploads (user)
    FeatureAnimatedBanner,
    /// Choosing a custom discriminator, ignored with the none tag style (user)
    FeatureCustomDiscriminator,
    /// Custom notification sounds (user)
    FeatureCustomNotificationSounds,
    /// Beta features (user)
    FeatureEarlyAccess,
    /// Using expressions in every guild (user)
    FeatureGlobalExpressions,
    /// Creating communities while the instance restricts it (user)
    FeatureGuildCreate,
    /// Higher video streaming quality (user)
    FeatureHigherVideoQuality,
    /// A different profile in each guild (user)
    FeaturePerGuildProfiles,
    /// A sound when joining voice (user)
    FeatureVoiceEntranceSounds,
    /// Days of history an unverified account sees while visiting a guild (user)
    GuildVisitHistoryDaysUnverified,
    /// Maximum size of each attachment in bytes (user and guild)
    MaxAttachmentFileSize,
    /// Maximum attachments per message (user and guild)
    MaxAttachmentsPerMessage,
    /// Maximum characters in a bio, at most 320 (user)
    MaxBioLength,
    /// Maximum bookmarked messages (user)
    MaxBookmarks,
    /// Maximum channels per category (guild)
    MaxChannelsPerCategory,
    /// Maximum custom profile backgrounds (user)
    MaxCustomBackgrounds,
    /// Maximum embeds per message (user and guild)
    MaxEmbedsPerMessage,
    /// Maximum meme tags to track (user)
    #[serde(rename = "max_favorite_meme_tags")]
    MaxSavedMediaTags,
    /// Maximum favorite memes (user)
    #[serde(rename = "max_favorite_memes")]
    MaxSavedMedia,
    /// Maximum members per group DM (user)
    MaxGroupDmRecipients,
    /// Maximum group DMs a user can own (user)
    MaxGroupDmsPerUser,
    /// Maximum channels per guild, counting forum and media channels but not threads (guild)
    MaxGuildChannels,
    /// Maximum emojis per guild (guild)
    MaxGuildEmojis,
    /// Alias of max_guild_emojis for older clients (guild)
    MaxGuildEmojisAnimated,
    /// Alias of max_guild_emojis for older clients (guild)
    MaxGuildEmojisAnimatedMore,
    /// Alias of max_guild_emojis for older clients (guild)
    MaxGuildEmojisStatic,
    /// Alias of max_guild_emojis for older clients (guild)
    MaxGuildEmojisStaticMore,
    /// Maximum active invites per guild (guild)
    MaxGuildInvites,
    /// Maximum members per guild (guild)
    MaxGuildMembers,
    /// Maximum roles per guild (guild)
    MaxGuildRoles,
    /// Maximum stickers per guild (guild)
    MaxGuildStickers,
    /// Alias of max_guild_stickers for older clients (guild)
    MaxGuildStickersMore,
    /// Maximum guilds a user can visit at once, halved for unverified accounts (user)
    MaxGuildVisits,
    /// Maximum guilds a user can join (user)
    MaxGuilds,
    /// Maximum characters per message (user and guild)
    MaxMessageLength,
    /// Maximum DM channels per user (user)
    MaxPrivateChannelsPerUser,
    /// Maximum distinct reactions per message (guild)
    MaxReactionsPerMessage,
    /// Maximum friends and blocked users (user)
    MaxRelationships,
    /// Highest screen share bitrate in bits per second, 1000000 to 50000000 (user)
    MaxScreenShareBitrate,
    /// Maximum users per reaction on a message (guild)
    MaxUsersPerMessageReaction,
    /// Maximum voice message length in seconds (user and guild)
    MaxVoiceMessageDuration,
    /// Maximum webhooks per channel (guild)
    MaxWebhooksPerChannel,
    /// Maximum webhooks per guild (guild)
    MaxWebhooksPerGuild,
    /// Maximum sticker upload size in bytes, up to 104857600 (guild)
    StickerMaxSize,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentLimitsRuleFilters {
    /// Trait names that must all be present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<String>>,
    /// Guild feature names that must all be present
    #[serde(skip_serializing_if = "Option::is_none", rename = "guildFeatures")]
    pub guild_features: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentLimitsRule {
    /// Unique identifier for this limits rule.
    pub id: String,
    /// What selects the rule, omitted when it always matches
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<InstanceDiscoveryDocumentLimitsRuleFilters>,
    /// Values keyed by limit key. Negative values are ignored
    pub overrides: HashMap<LimitKey, i64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentLimits {
    /// Wire format version. Is always `2`.
    pub version: u32,
    /// Available trait definitions, e.g. "premium".
    #[serde(rename = "traitDefinitions")]
    pub trait_definitions: Vec<String>,
    pub rules: Vec<InstanceDiscoveryDocumentLimitsRule>,
    /// Hash of the default limit values for cache invalidation.
    #[serde(rename = "defaultsHash")]
    pub defaults_hash: String,
}
