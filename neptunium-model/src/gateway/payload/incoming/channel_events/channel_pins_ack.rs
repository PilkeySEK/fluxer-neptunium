use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::ChannelMarker},
    time::timestamp::{Timestamp, representations::Iso8601},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelPinsAck {
    pub channel_id: Id<ChannelMarker>,
    pub timestamp: Timestamp<Iso8601>,
}
