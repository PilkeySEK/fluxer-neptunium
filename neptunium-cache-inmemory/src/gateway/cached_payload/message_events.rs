use std::ops::{Deref, DerefMut};

use neptunium_model::{
    channel::ChannelType,
    gateway::payload::incoming::{
        MessageCreate, MessageDelete, MessageReactionAdd, MessageReactionRemove,
    },
    guild::Emoji,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
    },
};

use crate::{
    CacheValue, Cached, CachedGuildMember, CachedMessage, gateway::cached_payload::CachedPayload,
};

pub struct CachedMessageCreate {
    pub message: Cached<CachedMessage>,
    pub channel_type: ChannelType,
}

impl CachedPayload for CachedMessageCreate {
    type NonCached = MessageCreate;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        if let Some(channel) = cache.channels.get(&non_cached.channel_id) {
            let channel = channel.load();
            if let Some(last_message_id) = channel.last_message_id.get() {
                last_message_id.store(non_cached.id);
            } else {
                let _ = channel.last_message_id.set(non_cached.id.into());
            }
        }
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

impl Deref for CachedMessageCreate {
    type Target = Cached<CachedMessage>;
    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl DerefMut for CachedMessageCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.message
    }
}

#[derive(Clone, Debug)]
pub struct CachedMessageReactionAdd {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub emoji: Emoji,
    pub user_id: Id<UserMarker>,
    pub session_id: Option<String>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub member: Option<Cached<CachedGuildMember>>,
}

impl CachedPayload for CachedMessageReactionAdd {
    type NonCached = MessageReactionAdd;

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
            message_id: non_cached.message_id,
            emoji: non_cached.emoji,
            user_id: non_cached.user_id,
            session_id: non_cached.session_id,
            guild_id: non_cached.guild_id,
            member,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CachedMessageReactionRemove {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub emoji: Emoji,
    pub user_id: Id<UserMarker>,
    pub session_id: Option<String>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub member: Option<Cached<CachedGuildMember>>,
}

impl CachedPayload for CachedMessageReactionRemove {
    type NonCached = MessageReactionRemove;

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
            message_id: non_cached.message_id,
            emoji: non_cached.emoji,
            user_id: non_cached.user_id,
            session_id: non_cached.session_id,
            guild_id: non_cached.guild_id,
            member,
        }
    }
}
