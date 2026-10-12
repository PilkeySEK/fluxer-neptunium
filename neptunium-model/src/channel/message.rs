use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    channel::{
        ChannelType,
        message::{attachment::MessageAttachment, embed::Embed, nonce::Nonce},
        thread::ThreadChannel,
    },
    id::{
        Id,
        marker::{
            ChannelMarker, EmojiMarker, MessageMarker, RoleMarker, StickerMarker, WebhookMarker,
        },
    },
    misc::serde_bitflags,
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

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    Regular = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    /// Private call system message.
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    ChannelFollowAdd = 12,
    ThreadCreated = 18,
    Reply = 19,
    ThreadStarterMessage = 21,
}

impl MessageType {
    /// Whether a message of this type can be modified, pinned, unpinned or replied to.
    /// If this is `false`, the API will reject these actions.
    #[must_use]
    pub fn can_be_modified_pinned_or_replied_to(&self) -> bool {
        matches!(self, Self::Regular | Self::Reply)
    }

    /// Whether a message of this type can be deleted.
    /// Trying to delete a message that cannot be deleted will be rejected by the API.
    #[must_use]
    pub fn can_be_deleted(&self) -> bool {
        matches!(
            self,
            Self::Regular
                | Self::ChannelPinnedMessage
                | Self::UserJoin
                | Self::ChannelFollowAdd
                | Self::Reply
                | Self::ThreadCreated
        )
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct MessageFlags: u32 {
        /// Message was published to the channels that follow its announcement channel.
        const CROSSPOSTED = 1 << 0;
        /// Message is a copy delivered from a followed announcement channel.
        const IS_CROSSPOST = 1 << 1;
        /// Suppress rendering of embeds.
        const SUPPRESS_EMBEDS = 1 << 2;
        /// The published message this copy came from was deleted.
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        /// A thread was started from this message.
        const HAS_THREAD = 1 << 5;
        /// Some members of a mentioned role were not added to the thread.
        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
        /// Do not generate ordinary mention notifications.
        const SUPPRESS_NOTIFICATIONS = 1 << 12;
        /// Message has one voice recording attachment.
        const VOICE_MESSAGE = 1 << 13;
    }
}

impl Default for MessageFlags {
    fn default() -> Self {
        Self::empty()
    }
}

serde_bitflags! {MessageFlags, u32}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub id: Id<MessageMarker>,
    pub channel_id: Id<ChannelMarker>,
    pub author: PartialUser,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<Id<WebhookMarker>>,
    #[serde(rename = "type")]
    pub r#type: MessageType,
    pub flags: MessageFlags,
    pub content: String,
    pub timestamp: Timestamp<Iso8601>,
    pub edited_timestamp: Option<Timestamp<Iso8601>>,
    pub pinned: bool,
    /// Whether the message mentions everyone.
    pub mention_everyone: bool,
    /// Whether the message requested text-to-speech.
    /// Echoed from the create request, so a message read through any other operation always reports false.
    pub tts: bool,
    pub mentions: Vec<PartialUser>,
    pub mention_roles: Vec<Id<RoleMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_channels: Option<Vec<ChannelMention>>,
    /// Users referenced by non-notifying content, embed and snapshot text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<PartialUser>>,
    pub embeds: Vec<Embed>,
    pub attachments: Vec<MessageAttachment>,
    pub stickers: Vec<StickerItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<MessageReaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_snapshots: Option<Vec<MessageSnapshot>>,
    /// Echoed only in the create message response and its originating `MessageCreate` gateway dispatch event,
    /// and never stored on the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Nonce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call: Option<MessageCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<Box<Message>>,
    /// The thread started from this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<ThreadChannel>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StickerItem {
    pub id: Id<StickerMarker>,
    pub name: String,
    pub animated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageReactionEmoji {
    #[serde(default)]
    pub animated: bool,
    /// `None` when the emoji is a unicode emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<EmojiMarker>>,
    /// Either the name of the emoji or the unicode character for standard emojis.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageReaction {
    pub count: u32,
    pub emoji: MessageReactionEmoji,
    #[serde(default)]
    pub me: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ChannelMention {
    pub id: Id<ChannelMarker>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: ChannelType,
}
