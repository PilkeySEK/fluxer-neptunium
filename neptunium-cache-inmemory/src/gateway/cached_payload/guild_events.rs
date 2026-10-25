use neptunium_model::{
    gateway::{
        payload::incoming::{GuildCreate, GuildDelete},
        presence::Presence,
        voice_state::VoiceState,
    },
    guild::{
        Guild,
        member::GuildMember,
        properties::{GuildEmoji, GuildSticker},
    },
    id::{Id, marker::GuildMarker},
    time::timestamp::{Timestamp, representations::Iso8601},
};

use crate::{
    CacheValue, Cached, CachedChannel, CachedGuildMember, gateway::cached_payload::CachedPayload,
};

pub struct CachedGuildCreate {
    pub guild: Cached<Guild>,
    pub channels: Vec<Cached<CachedChannel>>,
    pub id: Id<GuildMarker>,
    pub member_count: u64,
    pub online_count: u64,
    pub stickers: Vec<GuildSticker>,
    pub emojis: Vec<GuildEmoji>,
    pub members: Vec<Cached<CachedGuildMember>>,
    pub presences: Vec<Presence>,
    pub voice_states: Vec<VoiceState>,
    pub joined_at: Timestamp<Iso8601>,
}

impl CachedPayload for CachedGuildCreate {
    type NonCached = GuildCreate;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        Self {
            guild: Guild::from(non_cached.properties).insert_and_return(cache),
            channels: non_cached
                .channels
                .into_iter()
                .map(|channel| CachedChannel::from_channel(channel, cache).insert_and_return(cache))
                .collect(),
            id: non_cached.id,
            member_count: non_cached.member_count,
            online_count: non_cached.online_count,
            stickers: non_cached.stickers,
            emojis: non_cached.emojis,
            members: non_cached
                .members
                .into_iter()
                .map(|member| {
                    CachedGuildMember::from_guild_member(member, non_cached.id, cache)
                        .insert_and_return(cache)
                })
                .collect(),
            presences: non_cached.presences,
            voice_states: non_cached.voice_states,
            joined_at: non_cached.joined_at,
        }
    }
}

impl CachedPayload for GuildDelete {
    type NonCached = Self;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        cache.guilds.invalidate(&non_cached.id);
        non_cached
    }
}

impl CachedPayload for CachedGuildMember {
    type NonCached = (GuildMember, Id<GuildMarker>);
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        Self::from_guild_member(non_cached.0, non_cached.1, cache)
    }
}
