pub mod client;
pub mod events;
pub mod exts;
mod internal;
pub use async_trait::async_trait;
pub use tokio::main;

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

pub mod prelude {
    pub use crate::{
        client::{Client, ClientConfig},
        events::{EventError, EventErrorKind, EventHandler, context::Context},
        exts::*,
    };
    pub use async_trait::async_trait;
    pub use neptunium_gateway::shard::config::{ShardConfig, ShardConfigBuilder};
}
