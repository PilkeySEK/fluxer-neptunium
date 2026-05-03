//! A Fluxer bot framework.
//!
//! # Example
//! ```no_run
//! use fluxer_neptunium::{prelude::*, cached_payload::CachedMessageCreate};
//! use std::sync::Arc;
//!
//! struct Handler;
//!
//! // async_trait is a re-export of fluxer_neptunium (from the `async_trait` crate).
//! #[async_trait]
//! impl EventHandler for Handler {
//!   async fn on_message_create(&self, _ctx: Context, message: Arc<CachedMessageCreate>) -> Result<(), EventError> {
//!     let message = message.load();
//!     let author = message.author.load();
//!     println!("{}#{}: {}", author.username, author.discriminator, message.content);
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
#[cfg(any(test, feature = "experimental-macros"))]
pub mod macros;
pub use async_trait::async_trait;
pub use neptunium_http as http;
pub use neptunium_model as model;

const VERSION: &str = unwrap_or(option_env!("CARGO_PKG_VERSION"), "unknown");
const LIBRARY_NAME: &str = "fluxer-neptunium";

/// Custom `unwrap_or` const implementation because the one from the standard library is not yet stable.
const fn unwrap_or(option: Option<&'static str>, default: &'static str) -> &'static str {
    if let Some(value) = option {
        value
    } else {
        default
    }
}

pub use neptunium_cache_inmemory as cache;
pub use neptunium_cache_inmemory::gateway::cached_payload;

pub mod prelude {
    pub use crate::{
        client::{Client, ClientConfig},
        events::{EventError, EventErrorKind, EventHandler, context::Context},
        exts::*,
    };
    pub use async_trait::async_trait;
    pub use neptunium_cache_inmemory::Cached;
    pub use neptunium_gateway::shard::config::{ShardConfig, ShardConfigBuilder};
    pub use neptunium_model::gateway::intents::GatewayEventFlags;
}
