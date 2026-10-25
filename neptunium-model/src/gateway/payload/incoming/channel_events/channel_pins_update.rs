use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::ChannelMarker},
    time::timestamp::{Timestamp, representations::Iso8601},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelPinsUpdate {
    pub channel_id: Id<ChannelMarker>,
    /// Null if no pins remain.
    pub last_pin_timestamp: Option<Timestamp<Iso8601>>,
}
