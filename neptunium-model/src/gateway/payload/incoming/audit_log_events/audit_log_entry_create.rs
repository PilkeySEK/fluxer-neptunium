use serde::{Deserialize, Serialize};

use crate::{
    guild::audit_log::GuildAuditLogEntry,
    id::{Id, marker::GuildMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildAuditLogEntryCreate {
    #[serde(flatten)]
    pub audit_log_entry: GuildAuditLogEntry,
    pub guild_id: Id<GuildMarker>,
}
