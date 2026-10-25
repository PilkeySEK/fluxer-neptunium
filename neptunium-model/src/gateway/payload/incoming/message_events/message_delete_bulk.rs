use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageDeleteBulk {
    pub ids: Vec<Id<MessageMarker>>,
    pub channel_id: Id<ChannelMarker>,
}
