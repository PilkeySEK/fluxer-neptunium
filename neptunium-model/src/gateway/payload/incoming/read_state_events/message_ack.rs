use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageAck {
    pub channel_id: Id<ChannelMarker>,
    /// ID of the last read message.
    pub message_id: Id<MessageMarker>,
    /// Remaining unread mention count after this acknowledgement.
    pub mention_count: u64,
    /// Whether this was a manual acknowledgement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<bool>,
}
