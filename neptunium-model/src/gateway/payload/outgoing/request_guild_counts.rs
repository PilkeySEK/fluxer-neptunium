use bon::Builder;
use serde::Serialize;

use crate::id::{Id, marker::GuildMarker};

/// Request the member and online count of the specified guilds.
#[derive(Serialize, Clone, Debug, Builder)]
pub struct RequestGuildCounts {
    /// The guild IDs, max 100.
    pub guild_ids: Vec<Id<GuildMarker>>,
    /// A nonce for this request to identify the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}
