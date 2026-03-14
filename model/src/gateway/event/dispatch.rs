use serde::{Deserialize, Serialize};

use crate::gateway::payload::incoming::{
    guild_create::GuildCreate, guild_delete::GuildDelete, guild_emojis_update::GuildEmojisUpdate,
    message_create::MessageCreate, message_reaction_add::MessageReactionAdd,
    message_reaction_remove::MessageReactionRemove,
    message_reaction_remove_all::MessageReactionRemoveAll,
    message_reaction_remove_emoji::MessageReactionRemoveEmoji, ready::Ready,
    typing_start::TypingStart,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "t", content = "d", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchEvent {
    Ready(Ready),
    GuildDelete(GuildDelete),
    GuildCreate(GuildCreate),
    TypingStart(TypingStart),
    MessageCreate(MessageCreate),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionRemove(MessageReactionRemove),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmoji),
    MessageReactionRemoveAll(MessageReactionRemoveAll),
    GuildEmojisUpdate(GuildEmojisUpdate),
    // TODO: Other variants
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DispatchEventPayload {
    #[serde(flatten)]
    pub event: DispatchEvent,
    #[serde(rename = "s")]
    pub sequence_number: u64,
}
