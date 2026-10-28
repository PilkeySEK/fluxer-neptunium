use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use neptunium_model::{
    channel::{ChannelType, message::MessageReactionEmoji},
    gateway::payload::incoming::{
        MessageCreate, MessageDelete, MessageReactionAdd, MessageReactionAddMany,
        MessageReactionRemove, ReactionAddition,
    },
    guild::Emoji,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
    },
};

use crate::{
    Cache, CacheValue, Cached, CachedGuildMember, CachedMessage,
    gateway::cached_payload::CachedPayload,
};

pub struct CachedMessageCreate {
    pub message: Cached<CachedMessage>,
    pub channel_type: ChannelType,
    /// Group direct message nicknames, when the group has any.
    pub nicks: Option<HashMap<Id<UserMarker>, String>>,
    pub mention_here: Option<bool>,
    /// Guild the channel belongs to.
    pub guild_id: Option<Id<GuildMarker>>,
    pub member: Option<Cached<CachedGuildMember>>,
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
        Self {
            channel_type: non_cached.channel_type,
            nicks: non_cached.nicks,
            mention_here: non_cached.mention_here,
            guild_id: non_cached.guild_id,
            member: if let Some(guild_id) = non_cached.guild_id {
                non_cached.member.map(|member| {
                    CachedGuildMember {
                        id: non_cached.message.author.id,
                        user: non_cached.message.author.clone().insert_and_return(cache),
                        guild_id,
                        accent_color: member.accent_color,
                        avatar: member.avatar,
                        banner: member.banner,
                        communication_disabled_until: member.communication_disabled_until,
                        deaf: member.deaf,
                        joined_at: member.joined_at,
                        mute: member.mute,
                        nick: member.nick,
                        profile_flags: member.profile_flags,
                        roles: member.roles,
                    }
                    .insert_and_return(cache)
                })
            } else {
                None
            },
            message: CachedMessage::from_message(non_cached.message, cache)
                .insert_and_return(cache),
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedMessageReactionAddMany {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub reactions: Vec<CachedReactionAddition>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedReactionAddition {
    pub user_id: Id<UserMarker>,
    pub emoji: MessageReactionEmoji,
    pub member: Option<Cached<CachedGuildMember>>,
}

impl CachedPayload for CachedMessageReactionAddMany {
    type NonCached = MessageReactionAddMany;

    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        Self {
            channel_id: non_cached.channel_id,
            message_id: non_cached.message_id,
            guild_id: non_cached.guild_id,
            reactions: if let Some(guild_id) = non_cached.guild_id {
                non_cached
                    .reactions
                    .into_iter()
                    .map(|addition| (addition, guild_id, cache).into())
                    .collect()
            } else {
                if !non_cached.reactions.is_empty() {
                    tracing::warn!(
                        ?non_cached,
                        "`reactions` is not empty but the `guild_id` is Some"
                    );
                }
                Vec::new()
            },
        }
    }
}

impl From<(ReactionAddition, Id<GuildMarker>, &Arc<Cache>)> for CachedReactionAddition {
    fn from((value, guild_id, cache): (ReactionAddition, Id<GuildMarker>, &Arc<Cache>)) -> Self {
        Self {
            user_id: value.user_id,
            emoji: value.emoji,
            member: value.member.map(|member| {
                CachedGuildMember::from_guild_member(member, guild_id, cache)
                    .insert_and_return(cache)
            }),
        }
    }
}

impl From<CachedReactionAddition> for ReactionAddition {
    fn from(value: CachedReactionAddition) -> Self {
        Self {
            user_id: value.user_id,
            emoji: value.emoji,
            member: value.member.map(|member| member.clone_inner().into()),
        }
    }
}

impl From<CachedMessageReactionAddMany> for MessageReactionAddMany {
    fn from(value: CachedMessageReactionAddMany) -> Self {
        Self {
            channel_id: value.channel_id,
            message_id: value.message_id,
            guild_id: value.guild_id,
            reactions: value.reactions.into_iter().map(Into::into).collect(),
        }
    }
}
