use serde::Deserialize;

use crate::{
    guild::{
        audit_log::{
            change::AuditLogChange, event_type::AuditLogActionType,
            optional_entry_info::AuditLogOptionalEntryInfo,
        },
        webhook::AuditLogWebhook,
    },
    id::{
        Id,
        marker::{AuditLogEntryMarker, GenericMarker, GuildMarker, UserMarker},
    },
    user::UserPartial,
};

pub mod change;
pub mod change_key;
pub mod event_type;
pub mod optional_entry_info;

#[derive(Clone, Debug, Deserialize)]
pub struct AuditLogEntry {
    /// Type of event to cause the entry.
    pub action_type: AuditLogActionType,
    /// List of changes included in the entry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changes: Vec<AuditLogChange>,
    /// ID of the server where the entry was added.
    ///
    /// This is **only** available when receiving the event in
    /// [`GuildAuditLogEntryCreate`].
    ///
    /// [`GuildAuditLogEntryCreate`]: crate::gateway::payload::incoming::GuildAuditLogEntryCreate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// ID of the entire entry.
    pub id: Id<AuditLogEntryMarker>,
    /// Optional information about the entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AuditLogOptionalEntryInfo>,
    /// Optional application- or user-attached reason for the action that caused
    /// the entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// ID of the target entity.
    pub target_id: Option<Id<GenericMarker>>,
    /// ID of the [user] that performed the action.
    ///
    /// [user]: crate::user::User
    pub user_id: Option<Id<UserMarker>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildAuditLogs {
    pub audit_log_entries: Vec<AuditLogEntry>,
    // TODO: Is this actually UserPartial?
    pub users: Vec<UserPartial>,
    pub webhooks: Vec<AuditLogWebhook>,
}
