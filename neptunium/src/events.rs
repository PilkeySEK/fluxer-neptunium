use async_trait::async_trait;
use fluxer_model::gateway::payload::incoming::{message_create::MessageCreate, ready::Ready};

use crate::events::context::Context;

pub mod context;

#[expect(unused)]
#[async_trait]
pub trait EventHandler: Send {
    #[inline]
    async fn on_ready(&mut self, ctx: Context, data: Ready) {}
    #[inline]
    async fn on_message(&mut self, ctx: Context, data: MessageCreate) {}
}
