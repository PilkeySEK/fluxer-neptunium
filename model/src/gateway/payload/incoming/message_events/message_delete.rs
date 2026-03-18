use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{ChannelMarker, MessageMarker, UserMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageDelete {
    pub id: Id<MessageMarker>,
    pub channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<Id<UserMarker>>,
}
