use neptunium_model::{
    gateway::{payload::incoming::GuildCreate, presence::Presence, voice_state::VoiceState},
    guild::{
        Guild,
        member::GuildMember,
        properties::{GuildEmoji, GuildSticker},
    },
    id::{Id, marker::GuildMarker},
    time::timestamp::{Timestamp, representations::Iso8601},
};

use crate::{CacheValue, Cached, CachedChannel, gateway::cached_payload::CachedPayload};

pub struct CachedGuildCreate {
    pub guild: Cached<Guild>,
    pub channels: Vec<Cached<CachedChannel>>,
    pub id: Id<GuildMarker>,
    pub member_count: u64,
    pub online_count: u64,
    pub stickers: Vec<GuildSticker>,
    pub emojis: Vec<GuildEmoji>,
    pub members: Vec<GuildMember>,
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
            members: non_cached.members,
            presences: non_cached.presences,
            voice_states: non_cached.voice_states,
            joined_at: non_cached.joined_at,
        }
    }
}
