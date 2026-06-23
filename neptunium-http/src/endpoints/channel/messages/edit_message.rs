use bon::Builder;
use neptunium_model::{
    channel::message::{Message, MessageFlags, embed::MessageEmbed},
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};
use reqwest::Method;
use serde::Serialize;

use crate::{
    endpoints::{
        Endpoint,
        channel::{
            CreateMessageBody,
            messages::{allowed_mentions::AllowedMentions, attachment::AttachmentRequest},
        },
    },
    request::Request,
};

#[derive(Serialize, Clone, Debug, Builder)]
pub struct EditMessageBody {
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<MessageEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
}

#[derive(Builder, Clone, Debug)]
pub struct EditMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub body: EditMessageBody,
}

impl Endpoint for EditMessage {
    type Response = Message;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .path(format!(
                "/channels/{}/messages/{}",
                self.channel_id, self.message_id
            ))
            .body(serde_json::to_string(&self.body).unwrap())
            .build()
    }
}

impl From<String> for EditMessageBody {
    fn from(value: String) -> Self {
        Self {
            content: Some(value),
            embeds: None,
            attachments: None,
            allowed_mentions: None,
            flags: None,
        }
    }
}

impl From<&str> for EditMessageBody {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl From<CreateMessageBody> for EditMessageBody {
    fn from(value: CreateMessageBody) -> Self {
        Self {
            content: value.content,
            embeds: Some(value.embeds),
            attachments: Some(value.attachments),
            allowed_mentions: value.allowed_mentions,
            flags: Some(value.flags),
        }
    }
}

impl From<MessageEmbed> for EditMessageBody {
    fn from(value: MessageEmbed) -> Self {
        Self::from(vec![value])
    }
}

impl From<Vec<MessageEmbed>> for EditMessageBody {
    fn from(value: Vec<MessageEmbed>) -> Self {
        Self {
            content: None,
            embeds: Some(value),
            attachments: None,
            allowed_mentions: None,
            flags: None,
        }
    }
}
