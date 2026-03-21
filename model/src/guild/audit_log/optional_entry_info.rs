use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{ChannelMarker, GenericMarker, MessageMarker},
};

// Source: https://github.com/fluxerapp/fluxer/blob/d91388b979e7709575f929218dd053e081aa684e/packages/api/src/guild/services/GuildService.tsx#L90
/// Additional information for certain [`AuditLogEventType`]s.
///
/// [`AuditLogEventType`]: super::AuditLogEventType
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AuditLogOptionalEntryInfo {
    /// Channel in which the entities were targeted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Number of entities that were targeted, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /// Specified number of days' worth of inactivity members must have in order
    /// to be kicked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_member_days: Option<String>,
    /// ID of overwritten entity.
    pub id: Option<Id<GenericMarker>>,
    /// Number of members removed from a change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_removed: Option<u64>,
    /// ID of the affected message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Id<MessageMarker>>,
    /// Name of a role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    // TODO: Document all variants below this comment.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    // this is probably the channel type, but as a string instead of number (why, fluxer?????)
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<u64>,
    // TODO: Find out whether this is actually used or not
    // /// Type of integration which performed the action.
    // ///
    // /// The following events have this option:
    // ///
    // /// - [`AuditLogEventType::MemberKick`]
    // /// - [`AuditLogEventType::MemberRoleUpdate`]
    // ///
    // /// [`AuditLogEventType::MemberKick`]: super::AuditLogEventType::MemberKick
    // /// [`AuditLogEventType::MemberRoleUpdate`]: super::AuditLogEventType::MemberRoleUpdate
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub integration_type: Option<GuildIntegrationType>,
}
