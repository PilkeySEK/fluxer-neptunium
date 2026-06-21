use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    guild::{
        audit_log::{change::AuditLogChange, event_type::AuditLogActionType},
        webhook::AuditLogWebhook,
    },
    id::{
        Id,
        marker::{AuditLogEntryMarker, GenericMarker, GuildMarker, UserMarker},
    },
    user::PartialUser,
};

pub mod change;
pub mod change_key;
pub mod event_type;
// See comment in the module on why this is commented
// pub mod optional_entry_info;

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub options: Option<HashMap<String, serde_json::Value>>,
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

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct GuildAuditLogs {
    pub audit_log_entries: Vec<AuditLogEntry>,
    // TODO: Is this actually UserPartial?
    pub users: Vec<PartialUser>,
    pub webhooks: Vec<AuditLogWebhook>,
}
