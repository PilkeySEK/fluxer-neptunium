use serde::{Deserialize, Serialize};

use crate::{
    guild::Emoji,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReactionRemoveEmoji {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub emoji: Emoji,
}
