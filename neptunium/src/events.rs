use std::sync::Arc;

use async_trait::async_trait;
use fluxer_model::gateway::payload::incoming::{
    guild_create::GuildCreate, guild_delete::GuildDelete, guild_emojis_update::GuildEmojisUpdate,
    message_create::MessageCreate, message_reaction_add::MessageReactionAdd,
    message_reaction_remove::MessageReactionRemove,
    message_reaction_remove_all::MessageReactionRemoveAll,
    message_reaction_remove_emoji::MessageReactionRemoveEmoji, ready::Ready,
    typing_start::TypingStart,
};

use crate::events::context::Context;

pub mod context;

#[expect(unused)]
#[async_trait]
pub trait EventHandler: Send {
    async fn on_ready(&self, ctx: Context, data: Arc<Ready>) {}
    async fn on_message(&self, ctx: Context, data: Arc<MessageCreate>) {}
    async fn on_guild_create(&self, ctx: Context, data: Arc<GuildCreate>) {}
    async fn on_guild_delete(&self, ctx: Context, data: Arc<GuildDelete>) {}
    async fn on_typing_start(&self, ctx: Context, data: Arc<TypingStart>) {}
    async fn on_reaction_add(&self, ctx: Context, data: Arc<MessageReactionAdd>) {}
    async fn on_reaction_remove(&self, ctx: Context, data: Arc<MessageReactionRemove>) {}
    async fn on_reaction_remove_emoji(&self, ctx: Context, data: Arc<MessageReactionRemoveEmoji>) {}
    async fn on_reaction_remove_all(&self, ctx: Context, data: Arc<MessageReactionRemoveAll>) {}
    async fn on_guild_emojis_update(&self, ctx: Context, data: Arc<GuildEmojisUpdate>) {}
}
