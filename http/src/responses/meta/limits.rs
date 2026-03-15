use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LimitKey {
    AvatarMaxSize,
    EmojiMaxSize,
    FeatureAnimatedAvatar,
    FeatureAnimatedBanner,
    FeatureCustomDiscriminator,
    FeatureCustomNotificationSounds,
    FeatureEarlyAccess,
    FeatureGlobalExpressions,
    FeatureHigherVideoQuality,
    FeaturePerGuildProfiles,
    FeatureVoiceEntranceSounds,
    MaxAttachmentsPerMessage,
    MaxBioLength,
    MaxBookmarks,
    MaxChannelsPerCategory,
    MaxCreatedPacks,
    MaxCustomBackgrounds,
    MaxEmbedsPerMessage,
    MaxFavoriteMemeTags,
    MaxFavoriteMemes,
    MaxGroupDmRecipients,
    MaxGroupDmsPerUser,
    MaxGuildChannels,
    MaxGuildEmojisAnimatedMore,
    MaxGuildEmojisAnimated,
    MaxGuildEmojisStaticMore,
    MaxGuildEmojisStatic,
    MaxGuildInvites,
    MaxGuildMembers,
    MaxGuildRoles,
    MaxGuildStickersMore,
    MaxGuildStickers,
    MaxGuilds,
    MaxInstalledPacks,
    MaxAttachmentFileSize,
    MaxMessageLength,
    MaxPackExpressions,
    MaxPrivateChannelsPerUser,
    MaxReactionsPerMessage,
    MaxRelationships,
    MaxUsersPerMessageReaction,
    MaxVoiceMessageDuration,
    MaxWebhooksPerChannel,
    MaxWebhooksPerGuild,
    StickerMaxSize,
}

#[derive(Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentLimitsRuleFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "guildFeatures")]
    pub guild_features: Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentLimitsRule {
    /// Unique identifier for this limits rule.
    pub id: String,
    pub overrides: HashMap<LimitKey, i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<InstanceDiscoveryDocumentLimitsRuleFilters>,
}

#[derive(Deserialize, Clone, Debug)]
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
