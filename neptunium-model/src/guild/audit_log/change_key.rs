use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

// See https://github.com/fluxerapp/fluxer/blob/91ba0b096f6f39c717ebdadf48e39c394db21c8d/fluxer_app/src/components/modals/guild_tabs/GuildAuditLogTab.tsx#L100 ??

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditLogChangeKey {
    /// AFK voice channel for a guild.
    AfkChannelId,
    /// Timeout to cause a user to be moved to an AFK voice channel.
    AfkTimeout,
    /// Allowed permissions of a permission overwrite target.
    Allow,
    /// ID of an application.
    ApplicationId,
    /// Thread was archived or unarchived.
    Archived,
    /// Asset of a sticker.
    ///
    /// Empty string.
    Asset,
    /// Auto archive duration of a thread.
    AutoArchiveDuration,
    /// Availability of a sticker.
    Available,
    /// Hash of an avatar.
    AvatarHash,
    /// Hash of a guild banner.
    BannerHash,
    /// Bitrate of an audio channel.
    Bitrate,
    /// Channel for an invite code.
    ChannelId,
    /// Code of an invite.
    Code,
    /// Color of a role.
    Color,
    /// Permissions for a command were updated.
    CommandId,
    /// Member timeout state changed.
    CommunicationDisabledUntil,
    /// Whether a user is guild deafened.
    Deaf,
    /// Default auto archive duration for new threads.
    DefaultAutoArchiveDuration,
    /// Default message notification level for a guild.
    DefaultMessageNotifications,
    /// Denied permissions of a permission overwrite target.
    Deny,
    /// Description of a guild.
    Description,
    /// Hash of a guild's discovery splash.
    DiscoverySplashHash,
    /// Whether emoticons are enabled.
    EnableEmoticons,
    /// Entity type of guild scheduled event was changed.
    EntityType,
    /// Behavior of the expiration of an integration.
    ExpireBehavior,
    /// Grace period of the expiration of an integration.
    ExpireGracePeriod,
    /// Explicit content filter level of a guild.
    ExplicitContentFilter,
    /// Format type of a sticker.
    FormatType,
    /// Guild that a sticker is in.
    GuildId,
    /// Whether a role is hoisted.
    Hoist,
    /// Hash of a guild icon.
    IconHash,
    /// ID of an entity.
    Id,
    /// Hash of a guild scheduled event cover.
    ImageHash,
    /// Invitable state of a private thread.
    Invitable,
    /// ID of the user who created an invite.
    InviterId,
    /// Channel ID for a scheduled event changed.
    Location,
    /// Thread was locked or unlocked.
    Locked,
    /// Maximum age of an invite.
    MaxAge,
    /// Maximum uses of an invite.
    MaxUses,
    /// Whether a role can be mentioned in a message.
    Mentionable,
    /// Multi-Factor Authentication level required of a guild's moderators.
    MfaLevel,
    /// Whether a user is guild muted.
    Mute,
    /// Name of an entity such as a channel or role.
    Name,
    /// Nickname of a member.
    Nick,
    /// Whether a channel is NSFW.
    Nsfw,
    /// NSFW level of a guild.
    NsfwLevel,
    /// ID of the owner of a guild.
    OwnerId,
    /// Permission overwrites on a channel changed.
    PermissionOverwrites,
    /// Default permissions of a role.
    Permissions,
    /// Position of an entity such as a channel or role.
    Position,
    /// Preferred locale of a guild.
    PreferredLocale,
    /// Privacy level of a stage instance.
    PrivacyLevel,
    /// Number of days' worth of inactivity for a guild prune.
    PruneDeleteDays,
    /// ID of a guild's public updates channel.
    PublicUpdatesChannelId,
    /// Ratelimit per user in a textual channel.
    RateLimitPerUser,
    /// Region of a guild changed.
    Region,
    // /// Role added to a user.
    // #[serde(rename = "$add")]
    // RoleAdded,
    // /// Role removed from a user.
    // #[serde(rename = "$remove")]
    // RoleRemoved,
    /// User roles updated.
    Roles,
    /// ID of a guild's rules channel.
    RulesChannelId,
    /// Hash of a guild's splash.
    SplashHash,
    /// Status of guild scheduled event was changed.
    Status,
    /// ID of a guild's system channel.
    SystemChannelId,
    /// Related emoji of a sticker.
    Tags,
    /// Whether an invite is temporary.
    Temporary,
    /// Topic of a textual channel.
    Topic,
    /// Type of a created entity.
    Type,
    /// Role unicode emoji.
    UnicodeEmoji,
    /// Maximum number of users in a voice channel.
    UserLimit,
    /// Number of uses of an invite.
    Uses,
    /// Code of a guild's vanity invite.
    VanityUrlCode,
    /// Required verification level of new members in a guild.
    VerificationLevel,
    /// Channel ID of a widget.
    WidgetChannelId,
    /// Whether a widget is enabled.
    WidgetEnabled,
}

impl AuditLogChangeKey {
    /// Raw name of the key.
    ///
    /// The raw names of keys are in `snake_case` form.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::AfkChannelId => "afk_channel_id",
            Self::AfkTimeout => "afk_timeout",
            Self::Allow => "allow",
            Self::ApplicationId => "application_id",
            Self::Archived => "archived",
            Self::Asset => "asset",
            Self::AutoArchiveDuration => "auto_archive_duration",
            Self::Available => "available",
            Self::AvatarHash => "avatar_hash",
            Self::BannerHash => "banner_hash",
            Self::Bitrate => "bitrate",
            Self::ChannelId => "channel_id",
            Self::Code => "code",
            Self::Color => "color",
            Self::CommandId => "command_id",
            Self::CommunicationDisabledUntil => "communication_disabled_until",
            Self::Deaf => "deaf",
            Self::DefaultAutoArchiveDuration => "default_auto_archive_duration",
            Self::DefaultMessageNotifications => "default_message_notifications",
            Self::Deny => "deny",
            Self::Description => "description",
            Self::DiscoverySplashHash => "discovery_splash_hash",
            Self::EnableEmoticons => "enable_emoticons",
            Self::EntityType => "entity_type",
            Self::ExpireBehavior => "expire_behavior",
            Self::ExpireGracePeriod => "expire_grace_period",
            Self::ExplicitContentFilter => "explicit_content_filter",
            Self::FormatType => "format_type",
            Self::GuildId => "guild_id",
            Self::Hoist => "hoist",
            Self::IconHash => "icon_hash",
            Self::Id => "id",
            Self::ImageHash => "image_hash",
            Self::Invitable => "invitable",
            Self::InviterId => "inviter_id",
            Self::Location => "location",
            Self::Locked => "locked",
            Self::MaxAge => "max_age",
            Self::MaxUses => "max_uses",
            Self::Mentionable => "mentionable",
            Self::MfaLevel => "mfa_level",
            Self::Mute => "mute",
            Self::Name => "name",
            Self::Nick => "nick",
            Self::Nsfw => "nsfw",
            Self::NsfwLevel => "nsfw_level",
            Self::OwnerId => "owner_id",
            Self::PermissionOverwrites => "permission_overwrites",
            Self::Permissions => "permissions",
            Self::Position => "position",
            Self::PreferredLocale => "preferred_locale",
            Self::PrivacyLevel => "privacy_level",
            Self::PruneDeleteDays => "prune_delete_days",
            Self::PublicUpdatesChannelId => "public_updates_channel_id",
            Self::RateLimitPerUser => "rate_limit_per_user",
            Self::Region => "region",
            // Self::RoleAdded => "$add",
            // Self::RoleRemoved => "$remove",
            Self::Roles => "roles",
            Self::RulesChannelId => "rules_channel_id",
            Self::SplashHash => "splash_hash",
            Self::Status => "status",
            Self::SystemChannelId => "system_channel_id",
            Self::Tags => "tags",
            Self::Temporary => "temporary",
            Self::Topic => "topic",
            Self::Type => "type",
            Self::UnicodeEmoji => "unicode_emoji",
            Self::UserLimit => "user_limit",
            Self::Uses => "uses",
            Self::VanityUrlCode => "vanity_url_code",
            Self::VerificationLevel => "verification_level",
            Self::WidgetChannelId => "widget_channel_id",
            Self::WidgetEnabled => "widget_enabled",
        }
    }
}

impl Display for AuditLogChangeKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}
