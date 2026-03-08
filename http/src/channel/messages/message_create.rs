use serde::Serialize;

use fluxer_model::{
    channel::message::{MessageFlags, embed::MessageEmbed},
    id::{Id, marker::StickerMarker},
};

use crate::channel::messages::{
    allowed_mentions::AllowedMentions, attachment::AttachmentRequest,
    message_reference::MessageReference,
};

/// [Source](https://github.com/fluxerapp/fluxer/blob/03813bbe17db008452f0f1be3090a7d2970a5447/packages/schema/src/domains/message/MessageRequestSchemas.tsx#L247)
#[derive(Serialize, Clone, Debug)]
pub struct CreateMessageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    pub embeds: Vec<MessageEmbed>,
    pub attachments: Vec<AttachmentRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    pub flags: MessageFlags,
    pub nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite_meme_id: Option<String>, // TODO make this be Id<...>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_ids: Option<Vec<Id<StickerMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}

pub struct MessageBuilder {
    body: CreateMessageBody,
}

impl MessageBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            body: CreateMessageBody {
                content: None,
                embeds: vec![],
                attachments: vec![],
                message_reference: None,
                allowed_mentions: None,
                flags: MessageFlags::empty(),
                nonce: String::from("-"),
                favorite_meme_id: None,
                sticker_ids: None,
                tts: None,
            },
        }
    }

    #[must_use]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = Some(content.into());
        self
    }

    #[must_use]
    pub fn reference(mut self, reference: MessageReference) -> Self {
        self.body.message_reference = Some(reference);
        self
    }

    // TODO: All the other methods

    #[must_use]
    pub fn build(self) -> CreateMessageBody {
        self.body
    }
}
