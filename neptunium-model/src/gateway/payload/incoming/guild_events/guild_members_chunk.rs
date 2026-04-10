use serde::Deserialize;

use crate::{
    gateway::presence::Presence,
    guild::member::GuildMember,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMembersChunk {
    pub guild_id: Id<GuildMarker>,
    pub members: Vec<GuildMember>,
    pub chunk_index: u64,
    pub chunk_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_found: Option<Vec<Id<UserMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presences: Option<Presence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}
