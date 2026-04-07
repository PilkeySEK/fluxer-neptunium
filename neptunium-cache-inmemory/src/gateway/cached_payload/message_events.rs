use neptunium_model::{
    channel::ChannelType,
    gateway::payload::incoming::{MessageCreate, MessageDelete},
};

use crate::{CacheValue, Cached, CachedMessage, gateway::cached_payload::CachedPayload};

pub struct CachedMessageCreate {
    pub message: Cached<CachedMessage>,
    pub channel_type: ChannelType,
}

impl CachedPayload for CachedMessageCreate {
    type NonCached = MessageCreate;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        let message =
            CachedMessage::from_message(non_cached.message, cache).insert_and_return(cache);
        Self {
            message,
            channel_type: non_cached.channel_type,
        }
    }
}

impl CachedPayload for MessageDelete {
    type NonCached = Self;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        cache.messages.invalidate(&non_cached.id);
        non_cached
    }
}
