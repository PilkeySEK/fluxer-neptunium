use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReactionRemoveAll {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
}
