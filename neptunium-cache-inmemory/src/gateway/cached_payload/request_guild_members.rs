use neptunium_model::{
    gateway::{payload::incoming::GuildMembersChunk, presence::Presence},
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};

use crate::{CacheValue, Cached, CachedGuildMember, gateway::cached_payload::CachedPayload};

/// Sent in response to `RequestGuildMembers`.
#[derive(Clone, Debug)]
pub struct CachedGuildMembersChunk {
    pub guild_id: Id<GuildMarker>,
    pub members: Vec<Cached<CachedGuildMember>>,
    /// The chunk index in the expected chunks for this response (`0 <= chunk_index < chunk_count`).
    pub chunk_index: u64,
    /// The total number of expected chunks for this response.
    pub chunk_count: u64,
    pub not_found: Option<Vec<Id<UserMarker>>>,
    /// The presences of members, if requested.
    pub presences: Option<Vec<Presence>>,
    pub nonce: Option<String>,
}

impl CachedPayload for CachedGuildMembersChunk {
    type NonCached = GuildMembersChunk;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        let cached_members = non_cached
            .members
            .into_iter()
            .map(|member| {
                CachedGuildMember::from_guild_member(member, non_cached.guild_id, cache)
                    .insert_and_return(cache)
            })
            .collect::<Vec<_>>();

        Self {
            guild_id: non_cached.guild_id,
            members: cached_members,
            chunk_index: non_cached.chunk_index,
            chunk_count: non_cached.chunk_count,
            not_found: non_cached.not_found,
            presences: non_cached.presences,
            nonce: non_cached.nonce,
        }
    }
}
