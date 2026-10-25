use neptunium_model::{
    gateway::payload::incoming::TypingStart,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
    time::timestamp::{Timestamp, representations::UnixMillis},
};

use crate::{CacheValue, Cached, CachedGuildMember, gateway::cached_payload::CachedPayload};

#[derive(Clone, Debug)]
pub struct CachedTypingStart {
    pub channel_id: Id<ChannelMarker>,
    pub user_id: Id<UserMarker>,
    /// Timestamp of when the typing started.
    pub timestamp: Timestamp<UnixMillis>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub member: Option<Cached<CachedGuildMember>>,
}

impl CachedPayload for CachedTypingStart {
    type NonCached = TypingStart;

    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        let member = if let Some(member) = non_cached.member
            && let Some(guild_id) = non_cached.guild_id
        {
            Some(
                CachedGuildMember::from_guild_member(member, guild_id, cache)
                    .insert_and_return(cache),
            )
        } else {
            None
        };

        Self {
            channel_id: non_cached.channel_id,
            user_id: non_cached.user_id,
            timestamp: non_cached.timestamp,
            guild_id: non_cached.guild_id,
            member,
        }
    }
}
