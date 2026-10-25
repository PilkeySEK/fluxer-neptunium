use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    channel::Channel,
    gateway::voice_state::VoiceState,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PassiveUpdates {
    pub guild_id: Id<GuildMarker>,
    /// Map of channel id to most recent message id.
    pub channels: HashMap<Id<ChannelMarker>, Id<MessageMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_states: Option<Vec<VoiceState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_channels: Option<Vec<Channel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_channels: Option<Vec<Channel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_channel_ids: Option<Vec<Id<ChannelMarker>>>,
}
