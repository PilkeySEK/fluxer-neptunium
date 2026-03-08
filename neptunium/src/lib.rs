pub mod client;
pub mod events;
pub use async_trait::async_trait;
pub use tokio::main;

const VERSION: &str = unwrap_or(option_env!("CARGO_PKG_VERSION"), "unknown");

const fn unwrap_or(option: Option<&'static str>, default: &'static str) -> &'static str {
    if let Some(value) = option {
        value
    } else {
        default
    }
}

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::events::extensions::*;
    pub use crate::events::prelude::*;
    pub use fluxer_gateway::shard::config::{ShardConfig, ShardConfigBuilder};
}
