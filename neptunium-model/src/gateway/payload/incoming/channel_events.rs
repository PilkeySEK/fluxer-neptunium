mod channel_pins_ack;
mod channel_pins_update;
mod channel_update_bulk;

pub use channel_pins_ack::*;
pub use channel_pins_update::*;
pub use channel_update_bulk::*;

use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ChannelMemberCountsUpdate {
    pub counts: Vec<ChannelCountEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct ChannelCountEntry {
    pub guild_id: Id<GuildMarker>,
    pub channel_id: Id<ChannelMarker>,
    /// Number of members who can view the channel.
    pub member_count: usize,
    /// Number of members who can view the channel and are online.
    pub online_count: usize,
}
