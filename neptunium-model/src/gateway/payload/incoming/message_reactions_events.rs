mod message_reaction_add;
mod message_reaction_remove;
mod message_reaction_remove_all;
mod message_reaction_remove_emoji;

pub use message_reaction_add::*;
pub use message_reaction_remove::*;
pub use message_reaction_remove_all::*;
pub use message_reaction_remove_emoji::*;

use crate::{
    channel::message::MessageReactionEmoji,
    guild::member::GuildMember,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
    },
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageReactionAddMany {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    pub reactions: Vec<ReactionAddition>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ReactionAddition {
    pub user_id: Id<UserMarker>,
    pub emoji: MessageReactionEmoji,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,
}
