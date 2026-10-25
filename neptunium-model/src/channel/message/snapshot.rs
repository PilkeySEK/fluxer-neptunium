use serde::{Deserialize, Serialize};

use crate::{
    channel::message::{
        MessageFlags, MessageType, StickerItem, attachment::MessageAttachment, embed::Embed,
    },
    id::{
        Id,
        marker::{RoleMarker, UserMarker},
    },
    time::timestamp::{Timestamp, representations::Iso8601},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<MessageAttachment>>,
    // TODO: Maybe default to "" if no content is present, instead of doing Option<T>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<Timestamp<Iso8601>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    pub flags: MessageFlags,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_roles: Option<Vec<Id<RoleMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<Id<UserMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<StickerItem>>,
    pub timestamp: Timestamp<Iso8601>,
    #[serde(rename = "type")]
    pub r#type: MessageType,
}
