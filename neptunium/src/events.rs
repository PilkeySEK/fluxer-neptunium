use async_trait::async_trait;
use fluxer_model::gateway::payload::incoming::{
    guild_create::GuildCreate, guild_delete::GuildDelete, message_create::MessageCreate,
    ready::Ready, typing_start::TypingStart,
};

use crate::events::context::Context;

pub mod context;
pub mod extensions;

#[expect(unused)]
#[async_trait]
pub trait EventHandler: Send {
    #[inline]
    async fn on_ready(&mut self, ctx: Context, data: &Ready) {}
    #[inline]
    async fn on_message(&mut self, ctx: Context, data: &MessageCreate) {}
    #[inline]
    async fn on_guild_create(&mut self, ctx: Context, data: &GuildCreate) {}
    #[inline]
    async fn on_guild_delete(&mut self, ctx: Context, data: &GuildDelete) {}
    #[inline]
    async fn on_typing_start(&mut self, ctx: Context, data: &TypingStart) {}
}

pub(crate) mod prelude {
    pub use fluxer_model::gateway::payload::incoming::guild_create::GuildCreate;
    pub use fluxer_model::gateway::payload::incoming::message_create::MessageCreate;
    pub use fluxer_model::gateway::payload::incoming::ready::Ready;
    pub use fluxer_model::gateway::payload::incoming::typing_start::TypingStart;
}
