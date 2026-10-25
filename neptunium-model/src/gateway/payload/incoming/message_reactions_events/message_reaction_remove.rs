use serde::{Deserialize, Serialize};

use crate::{
    guild::{Emoji, member::GuildMember},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReactionRemove {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub emoji: Emoji,
    pub user_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,
}
