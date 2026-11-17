use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ReadState {
    pub id: Id<ChannelMarker>,
    pub mention_count: i32,
    pub last_message_id: Option<Id<MessageMarker>>,
    // ISO8601
    pub last_pin_timestamp: Option<jiff::Timestamp>,
}
