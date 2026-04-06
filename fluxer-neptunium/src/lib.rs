//! A Fluxer bot framework.
//!
//! # Example
//! ```no_run
//! use fluxer_neptunium::{prelude::*, model::gateway::payload::incoming::MessageCreate};
//! use std::sync::Arc;
//!
//! struct Handler;
//!
//! // async_trait is a re-export of fluxer_neptunium (from the `async_trait` crate).
//! #[async_trait]
//! impl EventHandler for Handler {
//!   async fn on_message_create(&self, _ctx: Context, message: Arc<MessageCreate>) -> Result<(), EventError> {
//!     println!("{}#{}: {}", message.author.username, message.author.discriminator, message.content);
//!     Ok(())
//!   }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!   let token = std::env::var("FLUXER_TOKEN").unwrap();
//!   let mut client = Client::new(token);
//!   client.register_event_handler(Handler);
//!   client.start().await.unwrap();
//! }
//! ```

pub mod client;
pub mod events;
pub mod exts;
mod internal;
pub use async_trait::async_trait;

pub use neptunium_http as http;
pub use neptunium_model as model;

const VERSION: &str = unwrap_or(option_env!("CARGO_PKG_VERSION"), "unknown");

/// Custom `unwrap_or` implementation because the one from the standard library is not yet stable.
const fn unwrap_or(option: Option<&'static str>, default: &'static str) -> &'static str {
    if let Some(value) = option {
        value
    } else {
        default
    }
}

pub use neptunium_cache_inmemory::gateway::cached_payload;

pub mod prelude {
    pub use crate::{
        client::{Client, ClientConfig},
        events::{EventError, EventErrorKind, EventHandler, context::Context},
        exts::*,
    };
    pub use async_trait::async_trait;
    pub use neptunium_gateway::shard::config::{ShardConfig, ShardConfigBuilder};
    pub use neptunium_model::gateway::intents::GatewayEventFlags;
}
