use serde::{Deserialize, Serialize};

use crate::{
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
    time::timestamp::{Timestamp, representations::Iso8601},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ReadState {
    pub id: Id<ChannelMarker>,
    pub mention_count: i32,
    pub last_message_id: Option<Id<MessageMarker>>,
    pub last_pin_timestamp: Option<Timestamp<Iso8601>>,
}
