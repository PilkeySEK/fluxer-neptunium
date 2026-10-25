use serde::{Deserialize, Serialize};

use crate::{
    gateway::voice_state::VoiceState,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker, UserMarker},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CallCreate {
    pub channel_id: Id<ChannelMarker>,
    // TODO: What is this?
    /// ID of the call message.
    pub message_id: Id<MessageMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub ringing: Vec<Id<UserMarker>>,
    pub voice_states: Vec<VoiceState>,
}
