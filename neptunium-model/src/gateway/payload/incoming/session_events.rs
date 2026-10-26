mod ready;
// TODO: Maybe move this event somewhere else
mod gateway_error;
mod hello;
mod resumed;

pub use gateway_error::*;
pub use hello::*;
pub use ready::*;
pub use resumed::*;

use crate::{
    gateway::presence::PresenceStatus,
    id::{Id, marker::GuildMarker},
    time::duration::{Duration, Seconds},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SessionPresence {
    /// Session identifier or `all` for the combined entry.
    pub session_id: String,
    pub status: PresenceStatus,
    pub afk: bool,
    pub mobile: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RateLimited {
    /// Is always `8`.
    pub opcode: u8,
    pub retry_after: Duration<Seconds>,
    pub meta: RateLimitedMeta,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RateLimitedMeta {
    pub guild_id: Id<GuildMarker>,
    // TODO: Type undocumented, need to check in source code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}
