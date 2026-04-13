use std::ops::{Deref, DerefMut};

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    channel::message::{attachment::MessageAttachment, embed::MessageEmbed, nonce::Nonce},
    id::{
        Id,
        marker::{
            ChannelMarker, EmojiMarker, MessageMarker, RoleMarker, StickerMarker, WebhookMarker,
        },
    },
    serde_bitflags,
    time::timestamp::{Timestamp, representations::Iso8601},
    user::PartialUser,
};

pub mod attachment;
mod call;
pub mod embed;
pub mod nonce;
mod reference;
mod snapshot;
pub use call::*;
pub use reference::*;
pub use snapshot::*;

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum MessageType {
    Regular = 0,
    UserAddedToConversation = 1,
    UserRemovedFromConversation = 2,
    /// Representing a call.
    Call = 3,
    ChannelNameChanged = 4,
    ChannelIconChanged = 5,
    MessagePinned = 6,
    UserJoin = 7,
    Reply = 19,
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct MessageFlags: u32 {
        const SUPPRESS_EMBEDS = 1 << 2;
        /// Will not trigger push or desktop notifications.
        const SUPPRESS_NOTIFICATIONS = 1 << 12;
        const VOICE_MESSAGE = 1 << 13;
        const COMPACT_ATTACHMENTS = 1 << 17;
    }
}

impl Default for MessageFlags {
    fn default() -> Self {
        Self::empty()
    }
}

serde_bitflags! {MessageFlags, u32}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    #[serde(flatten)]
    pub base: MessageBase,
    /// The message that this message is replying to or forwarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<MessageBase>,
}

impl Deref for Message {
    type Target = MessageBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReactionEmoji {
    #[serde(default)]
    pub animated: bool,
    /// `None` when the emoji is a unicode emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<EmojiMarker>>,
    /// Either the name of the emoji or the unicode character for standard emojis.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReaction {
    pub count: u32,
    pub emoji: MessageReactionEmoji,
    #[serde(default)]
    pub me: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageSticker {
    pub animated: bool,
    pub id: Id<StickerMarker>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<MessageAttachment>>,
    pub author: PartialUser,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call: Option<MessageCall>,
    pub channel_id: Id<ChannelMarker>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<Timestamp<Iso8601>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<MessageEmbed>>,
    pub flags: MessageFlags,
    pub id: Id<MessageMarker>,
    pub mention_everyone: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_roles: Option<Vec<Id<RoleMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<PartialUser>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    /// Snapshots of forwarded messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_snapshots: Option<Vec<MessageSnapshot>>,
    /// A client-provided value for message deduplication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Nonce>,
    pub pinned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<MessageReaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<MessageSticker>>,
    pub timestamp: Timestamp<Iso8601>,
    #[serde(default)]
    pub tts: bool,
    #[serde(rename = "type")]
    pub r#type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<Id<WebhookMarker>>,
}
