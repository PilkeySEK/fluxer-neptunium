use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::ChannelMarker};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelPinsAck {
    pub channel_id: Id<ChannelMarker>,
    // ISO8601
    pub timestamp: jiff::Timestamp,
}
