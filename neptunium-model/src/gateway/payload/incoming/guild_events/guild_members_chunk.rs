use serde::Deserialize;

use crate::{
    gateway::presence::Presence,
    guild::member::GuildMember,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};

/// Sent in response to `RequestGuildMembers`.
#[derive(Deserialize, Clone, Debug)]
pub struct GuildMembersChunk {
    pub guild_id: Id<GuildMarker>,
    pub members: Vec<GuildMember>,
    /// The chunk index in the expected chunks for this response (`0 <= chunk_index < chunk_count`)
    pub chunk_index: u64,
    /// The total number of expected chunks for this response.
    pub chunk_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_found: Option<Vec<Id<UserMarker>>>,
    /// The presences of members, if requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presences: Option<Vec<Presence>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}
