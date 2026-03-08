use serde::Serialize;

use fluxer_model::{
    channel::message::MessageReferenceType,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker},
    },
};

#[derive(Serialize, Clone, Debug)]
pub struct MessageReference {
    pub message_id: Id<MessageMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(rename = "type")]
    pub r#type: MessageReferenceType,
}
