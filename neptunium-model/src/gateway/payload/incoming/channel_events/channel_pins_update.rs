use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::ChannelMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelPinsUpdate {
    pub channel_id: Id<ChannelMarker>,
    // ISO8601
    /// Null if no pins remain.
    pub last_pin_timestamp: Option<jiff::Timestamp>,
}
