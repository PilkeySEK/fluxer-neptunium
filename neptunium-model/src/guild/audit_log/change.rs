// https://github.com/twilight-rs/twilight/blob/main/twilight-model/src/guild/audit_log/change.rs

use crate::{
    channel::PermissionOverwrite,
    guild::{
        audit_log::change_key::AuditLogChangeKey,
        default_message_notification_level::DefaultMessageNotificationLevel,
        explicit_content_filter::ExplicitContentFilter,
        permissions::Permissions,
        properties::{GuildMfaLevel, GuildVerificationLevel, NsfwLevel},
    },
    id::{
        Id,
        marker::{
            ApplicationMarker, ChannelMarker, GenericMarker, GuildMarker, RoleMarker, UserMarker,
        },
    },
    time::timestamp::{Timestamp, representations::Iso8601},
};
use serde::{Deserialize, Serialize};

/// Minimal amount of information about an affected role.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AffectedRole {
    /// ID of the role.
    pub id: Id<RoleMarker>,
    /// Name of the role.
    pub name: String,
}

/// Value of a change which may be one of multiple types.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AuditLogChangeTypeValue {
    /// Value is an unsigned integer.
    Unsigned(u64),
    /// Value is a string.
    String(String),
}

/// Individual change within an `AuditLogEntry`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case", tag = "key")]
pub enum AuditLogChange {
    /// AFK channel ID was changed.
    AfkChannelId {
        /// New ID of the AFK channel.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<ChannelMarker>>,
        /// Old ID of the AFK channel.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<ChannelMarker>>,
    },
    /// Timeout to cause a user to be moved to an AFK voice channel.
    AfkTimeout {
        /// New timeout, in seconds.
        #[serde(rename = "new_value")]
        new: u64,
        /// Old timeout, in seconds.
        #[serde(rename = "old_value")]
        old: u64,
    },
    /// Allowed permissions of a permission overwrite target.
    Allow {
        /// New allowed permissions value.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Permissions>,
        /// Old allowed permissions value.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Permissions>,
    },
    /// ID of an application.
    ApplicationId {
        /// Application's ID.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<ApplicationMarker>>,
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<ApplicationMarker>>,
    },
    /// Thread is now archived/unarchived.
    Archived {
        /// Whether the thread is archived.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// Asset of a sticker.
    Asset {
        /// Empty string.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    // Threads are not yet implemented in Fluxer
    // /// Auto archive duration of a thread changed.
    // AutoArchiveDuration {
    //     /// New auto archive duration.
    //     #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
    //     new: Option<AutoArchiveDuration>,
    //     /// Old auto archive duration.
    //     #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
    //     old: Option<AutoArchiveDuration>,
    // },
    /// Availability of a sticker.
    Available {
        /// New availability.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Old availability.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// Hash of an avatar.
    AvatarHash {
        /// New hash of an avatar.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old hash of an avatar.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Hash of a guild banner.
    BannerHash {
        /// New hash of a guild's banner.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old hash of a guild's banner.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Bitrate of an audio channel.
    Bitrate {
        /// New bitrate.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Old bitrate.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// Channel for an invite code.
    ChannelId {
        /// New invite's channel.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<ChannelMarker>>,
        /// Old invite's channel.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<ChannelMarker>>,
    },
    /// Code of an invite.
    Code {
        /// New invite's code.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Color of a role.
    Color {
        /// New role color.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Old role color.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    // Commands are not yet implemented in Fluxer
    // /// Permissions for a command were updated
    // CommandId {
    //     /// New command permissions.
    //     #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
    //     new: Option<GuildCommandPermissions>,
    //     /// Old command permissions.
    //     #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
    //     old: Option<GuildCommandPermissions>,
    // },
    /// Member timeout state changed.
    CommunicationDisabledUntil {
        /// New timeout timestamp.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Timestamp<Iso8601>>,
        /// Old timeout timestamp.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Timestamp<Iso8601>>,
    },
    /// Whether a member is guild deafened.
    Deaf {
        /// Whether a member is now guild deafened.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    // Threads are not yet implemented in Fluxer
    // /// default auto archive duration for newly created threads changed.
    // DefaultAutoArchiveDuration {
    //     /// New auto archive duration.
    //     #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
    //     new: Option<AutoArchiveDuration>,
    //     /// Old auto archive duration.
    //     #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
    //     old: Option<AutoArchiveDuration>,
    // },
    /// Default message notification level for a guild.
    DefaultMessageNotifications {
        /// New default message notification level.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<DefaultMessageNotificationLevel>,
        /// Old default message notification level.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<DefaultMessageNotificationLevel>,
    },
    /// Denied permissions of a permission overwrite target.
    Deny {
        /// New denied permissions level.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Permissions>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Permissions>,
    },
    /// Description of a guild or sticker.
    Description {
        /// New guild description.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old guild description.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Hash of a guild's discovery splash.
    DiscoverySplashHash {
        /// New discovery splash hash.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old discovery splash hash.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Whether emoticons are enabled.
    EnableEmoticons {
        /// Whether emoticons are now enabled.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Whether emoticons were enabled.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// Entity type of guild scheduled event was changed.
    EntityType {
        /// New entity type.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// Behavior of the expiration of an integration.
    ExpireBehavior {
        /// New expiration behavior.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// Grace period of the expiration of an integration.
    ExpireGracePeriod {
        /// New expiration grace period.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// Explicit content filter level of a guild.
    ExplicitContentFilter {
        /// New explicit content filter level.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<ExplicitContentFilter>,
        /// Old explicit content filter level.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<ExplicitContentFilter>,
    },
    // TODO: Not sure if this is a thing in Fluxer
    // /// Format type of a sticker.
    // FormatType {
    //     /// New format type of a sticker.
    //     #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
    //     new: Option<StickerFormatType>,
    //     /// Old format type of a sticker.
    //     #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
    //     old: Option<StickerFormatType>,
    // },
    /// Guild that a sticker is in.
    GuildId {
        /// New guild that a sticker is in.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<GuildMarker>>,
        /// Old guild that a sticker is in.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<GuildMarker>>,
    },
    /// Whether a role is hoisted.
    Hoist {
        /// Whether a role is now hoisted.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Whether a role was hoisted.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// Hash of a guild icon.
    IconHash {
        /// New hash of a guild's icon.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old hash of a guild's icon.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// ID of an entity.
    Id {
        /// New entity's ID.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<GenericMarker>>,
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<GenericMarker>>,
    },
    /// Hash of a guild scheduled event cover.
    ImageHash {
        /// New hash of a guild's icon.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old hash of a guild's icon.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Invitable state of a private thread.
    Invitable {
        /// New threads invitable state.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Old threads invitable state.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// ID of the user who created an invite.
    InviterId {
        /// User ID.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<UserMarker>>,
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<UserMarker>>,
    },
    /// Location for a scheduled event changed.
    ///
    /// Can be an [`Id<ChannelMarker>`] or a [`String`].
    Location {
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Thread was locked or unlocked.
    Locked {
        /// Whether the thread is now locked.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// Maximum age of an invite.
    MaxAge {
        /// New maximum age.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// Maximum uses of an invite.
    MaxUses {
        /// New maximum uses.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// Whether a role can be mentioned in a message.
    Mentionable {
        /// Whether a role is now mentionable.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Whether a role was mentionable.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// Multi-Factor Authentication level required of a guild's moderators.
    MfaLevel {
        /// New MFA level of a guild.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<GuildMfaLevel>,
        /// Old MFA level of a guild.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<GuildMfaLevel>,
    },
    /// Whether a user is guild muted.
    Mute {
        /// Whether a member is now muted.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Whether a member was muted.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// Name of an entity such as a channel or role.
    Name {
        /// New entity name.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old entity name.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Nickname of a member.
    Nick {
        /// New member nickname.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old member nickname.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Whether a channel is NSFW.
    Nsfw {
        /// New state.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// NSFW level of a guild.
    NsfwLevel {
        /// New NSFW level.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<NsfwLevel>,
        /// Old NSFW level.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<NsfwLevel>,
    },
    /// ID of the owner of a guild.
    OwnerId {
        /// New owner's ID.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<UserMarker>>,
        /// Old owner's ID.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<UserMarker>>,
    },
    /// Permission overwrites on a channel changed.
    PermissionOverwrites {
        /// New set of overwrites.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Vec<PermissionOverwrite>>,
        /// Old set of overwrites.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Vec<PermissionOverwrite>>,
    },
    /// Default permissions of a role.
    Permissions {
        /// New set of permissions.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Permissions>,
        /// Old set of permissions.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Permissions>,
    },
    /// Position of an entity such as a channel or role.
    Position {
        /// New position value.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Old position value.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// Preferred locale of a guild.
    PreferredLocale {
        /// New preferred locale.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old preferred locale.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    // Fluxer doesn't have stages - yet?
    // /// Privacy level of a stage instance.
    // PrivacyLevel {
    //     /// New privacy level.
    //     #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
    //     new: Option<PrivacyLevel>,
    //     /// Old privacy level.
    //     #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
    //     old: Option<PrivacyLevel>,
    // },
    /// Number of days' worth of inactivity for a guild prune.
    PruneDeleteDays {
        /// Number of days.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// ID of a guild's public updates channel.
    PublicUpdatesChannelId {
        /// New public updates channel ID.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<ChannelMarker>>,
        /// Old public updates channel ID.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<ChannelMarker>>,
    },
    /// Ratelimit per user in a textual channel.
    RateLimitPerUser {
        /// New ratelimit, in seconds.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Old ratelimit, in seconds.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// Region of a guild changed.
    Region {
        /// New region.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    // /// Role was added to a user.
    // #[serde(rename = "$add")]
    // RoleAdded {
    //     /// Minimal information about a added role.
    //     #[serde(default, rename = "new_value", skip_serializing_if = "Vec::is_empty")]
    //     new: Vec<AffectedRole>,
    //     /// Previous state, if any.
    //     #[serde(default, rename = "old_value", skip_serializing_if = "Vec::is_empty")]
    //     old: Vec<AffectedRole>,
    // },
    // /// Role was removed from a user.
    // #[serde(rename = "$remove")]
    // RoleRemoved {
    //     /// Minimal information about a removed role.
    //     #[serde(default, rename = "new_value", skip_serializing_if = "Vec::is_empty")]
    //     new: Vec<AffectedRole>,
    //     /// Previous state, if any.
    //     #[serde(default, rename = "old_value", skip_serializing_if = "Vec::is_empty")]
    //     old: Vec<AffectedRole>,
    // },
    Roles {
        #[serde(default, rename = "new_value", skip_serializing_if = "Vec::is_empty")]
        new: Vec<Id<RoleMarker>>,
        #[serde(default, rename = "old_value", skip_serializing_if = "Vec::is_empty")]
        old: Vec<Id<RoleMarker>>,
    },
    /// Guild's rules channel.
    RulesChannelId {
        /// New rules channel.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<ChannelMarker>>,
        /// Old rules channel.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<ChannelMarker>>,
    },
    /// Hash of a guild's splash.
    SplashHash {
        /// Old hash of a guild's splash.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// New hash of a guild's splash.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Status of guild scheduled event was changed.
    Status {
        /// New status.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// ID of guild's system channel.
    SystemChannelId {
        /// New system channel ID.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<ChannelMarker>>,
        /// Old system channel ID.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<ChannelMarker>>,
    },
    /// Related emoji of a sticker.
    Tags {
        /// New related emoji of a sticker.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old related emoji of a sticker.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Whether an invite is temporary.
    Temporary {
        /// New temporary state.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// Topic of a textual channel.
    Topic {
        /// New topic.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old topic.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Type of a created entity.
    ///
    /// The value of a type is dependent on the entity. For example, a channel's
    /// type may be an integer while an integration's may be a string.
    Type {
        /// New target type.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<AuditLogChangeTypeValue>,
        /// Old target type.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<AuditLogChangeTypeValue>,
    },
    /// Unicode emoji of a role icon changed.
    UnicodeEmoji {
        /// New unicode emoji.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old target type.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Maximum number of users in a voice channel.
    UserLimit {
        /// New limit.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Old limit.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// Number of uses of an invite.
    Uses {
        /// Number of uses.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<u64>,
        /// Previous state, if any.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<u64>,
    },
    /// Code of a guild's vanity invite.
    VanityUrlCode {
        /// New vanity URL code.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old vanity URL code.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Required verification level of new members in a guild.
    VerificationLevel {
        /// New verification level.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<GuildVerificationLevel>,
        /// Old verification level.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<GuildVerificationLevel>,
    },
    /// Channel ID of a widget.
    WidgetChannelId {
        /// New channel ID.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<Id<ChannelMarker>>,
        /// Old channel ID.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<Id<ChannelMarker>>,
    },
    /// Whether a widget is enabled.
    WidgetEnabled {
        /// New state of a widget being enabled.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<bool>,
        /// Old state of a widget being enabled.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// Other type of change not covered by other variants.
    #[serde(other)]
    Other,
}

impl AuditLogChange {
    /// Key of an audit log change.
    #[must_use]
    pub const fn key(&self) -> Option<AuditLogChangeKey> {
        Some(match self {
            Self::AfkChannelId { .. } => AuditLogChangeKey::AfkChannelId,
            Self::AfkTimeout { .. } => AuditLogChangeKey::AfkTimeout,
            Self::Allow { .. } => AuditLogChangeKey::Allow,
            Self::ApplicationId { .. } => AuditLogChangeKey::ApplicationId,
            Self::Archived { .. } => AuditLogChangeKey::Archived,
            Self::Asset { .. } => AuditLogChangeKey::Asset,
            // Self::AutoArchiveDuration { .. } => AuditLogChangeKey::AutoArchiveDuration,
            Self::Available { .. } => AuditLogChangeKey::Available,
            Self::AvatarHash { .. } => AuditLogChangeKey::AvatarHash,
            Self::BannerHash { .. } => AuditLogChangeKey::BannerHash,
            Self::Bitrate { .. } => AuditLogChangeKey::Bitrate,
            Self::ChannelId { .. } => AuditLogChangeKey::ChannelId,
            Self::Code { .. } => AuditLogChangeKey::Code,
            Self::Color { .. } => AuditLogChangeKey::Color,
            // Self::CommandId { .. } => AuditLogChangeKey::CommandId,
            Self::CommunicationDisabledUntil { .. } => {
                AuditLogChangeKey::CommunicationDisabledUntil
            }
            Self::Deaf { .. } => AuditLogChangeKey::Deaf,
            // Self::DefaultAutoArchiveDuration { .. } => {
            //     AuditLogChangeKey::DefaultAutoArchiveDuration
            // }
            Self::DefaultMessageNotifications { .. } => {
                AuditLogChangeKey::DefaultMessageNotifications
            }
            Self::Deny { .. } => AuditLogChangeKey::Deny,
            Self::Description { .. } => AuditLogChangeKey::Description,
            Self::DiscoverySplashHash { .. } => AuditLogChangeKey::DiscoverySplashHash,
            Self::EnableEmoticons { .. } => AuditLogChangeKey::EnableEmoticons,
            Self::EntityType { .. } => AuditLogChangeKey::EntityType,
            Self::ExpireBehavior { .. } => AuditLogChangeKey::ExpireBehavior,
            Self::ExpireGracePeriod { .. } => AuditLogChangeKey::ExpireGracePeriod,
            Self::ExplicitContentFilter { .. } => AuditLogChangeKey::ExplicitContentFilter,
            // Self::FormatType { .. } => AuditLogChangeKey::FormatType,
            Self::GuildId { .. } => AuditLogChangeKey::GuildId,
            Self::Hoist { .. } => AuditLogChangeKey::Hoist,
            Self::IconHash { .. } => AuditLogChangeKey::IconHash,
            Self::Id { .. } => AuditLogChangeKey::Id,
            Self::ImageHash { .. } => AuditLogChangeKey::ImageHash,
            Self::Invitable { .. } => AuditLogChangeKey::Invitable,
            Self::InviterId { .. } => AuditLogChangeKey::InviterId,
            Self::Location { .. } => AuditLogChangeKey::Location,
            Self::Locked { .. } => AuditLogChangeKey::Locked,
            Self::MaxAge { .. } => AuditLogChangeKey::MaxAge,
            Self::MaxUses { .. } => AuditLogChangeKey::MaxUses,
            Self::Mentionable { .. } => AuditLogChangeKey::Mentionable,
            Self::MfaLevel { .. } => AuditLogChangeKey::MfaLevel,
            Self::Mute { .. } => AuditLogChangeKey::Mute,
            Self::Name { .. } => AuditLogChangeKey::Name,
            Self::Nick { .. } => AuditLogChangeKey::Nick,
            Self::Nsfw { .. } => AuditLogChangeKey::Nsfw,
            Self::NsfwLevel { .. } => AuditLogChangeKey::NsfwLevel,
            Self::OwnerId { .. } => AuditLogChangeKey::OwnerId,
            Self::PermissionOverwrites { .. } => AuditLogChangeKey::PermissionOverwrites,
            Self::Permissions { .. } => AuditLogChangeKey::Permissions,
            Self::Position { .. } => AuditLogChangeKey::Position,
            Self::PreferredLocale { .. } => AuditLogChangeKey::PreferredLocale,
            // Self::PrivacyLevel { .. } => AuditLogChangeKey::PrivacyLevel,
            Self::PruneDeleteDays { .. } => AuditLogChangeKey::PruneDeleteDays,
            Self::PublicUpdatesChannelId { .. } => AuditLogChangeKey::PublicUpdatesChannelId,
            Self::RateLimitPerUser { .. } => AuditLogChangeKey::RateLimitPerUser,
            Self::Region { .. } => AuditLogChangeKey::Region,
            // Self::RoleAdded { .. } => AuditLogChangeKey::RoleAdded,
            // Self::RoleRemoved { .. } => AuditLogChangeKey::RoleRemoved,
            Self::Roles { .. } => AuditLogChangeKey::Roles,
            Self::RulesChannelId { .. } => AuditLogChangeKey::RulesChannelId,
            Self::SplashHash { .. } => AuditLogChangeKey::SplashHash,
            Self::Status { .. } => AuditLogChangeKey::Status,
            Self::SystemChannelId { .. } => AuditLogChangeKey::SystemChannelId,
            Self::Tags { .. } => AuditLogChangeKey::Tags,
            Self::Temporary { .. } => AuditLogChangeKey::Temporary,
            Self::Topic { .. } => AuditLogChangeKey::Topic,
            Self::Type { .. } => AuditLogChangeKey::Type,
            Self::UnicodeEmoji { .. } => AuditLogChangeKey::UnicodeEmoji,
            Self::UserLimit { .. } => AuditLogChangeKey::UserLimit,
            Self::Uses { .. } => AuditLogChangeKey::Uses,
            Self::VanityUrlCode { .. } => AuditLogChangeKey::VanityUrlCode,
            Self::VerificationLevel { .. } => AuditLogChangeKey::VerificationLevel,
            Self::WidgetChannelId { .. } => AuditLogChangeKey::WidgetChannelId,
            Self::WidgetEnabled { .. } => AuditLogChangeKey::WidgetEnabled,
            Self::Other => return None,
        })
    }
}
