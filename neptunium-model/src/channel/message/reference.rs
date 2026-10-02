use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker, MessageMarker},
};

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, Default)]
#[repr(u8)]
pub enum MessageReferenceType {
    #[default]
    Reply = 0,
    Forward = 1,
}

#[derive(Serialize, Deserialize, Clone, Debug, Builder)]
pub struct MessageReference {
    /// The ID of the channel containing the referenced message.
    pub channel_id: Option<Id<ChannelMarker>>,
    /// The ID of the guild containing the referenced message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// The ID of the referenced message.
    /// Not present when the message is of type `ChannelFollowAdd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Id<MessageMarker>>,
    #[builder(default)]
    #[serde(rename = "type", default)]
    pub r#type: MessageReferenceType,
}
