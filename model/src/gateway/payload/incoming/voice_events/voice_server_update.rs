use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoiceServerUpdate {
    pub token: String,
    pub endpoint: String,
    pub connection_id: String,
    /// Present for guild voice channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Present for DM calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
}
