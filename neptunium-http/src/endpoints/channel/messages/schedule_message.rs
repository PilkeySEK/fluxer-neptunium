use bon::Builder;
use neptunium_model::{
    channel::message::{
        Message, MessageFlags, MessageReference, MessageSticker, attachment::MessageAttachment,
        embed::MessageEmbed, nonce::Nonce,
    },
    id::{
        Id,
        marker::{ChannelMarker, GenericMarker, ScheduledMessageMarker, StickerMarker},
    },
    time::timestamp::{Timestamp, representations::Iso8601},
};
use reqwest::Method;
use serde::Deserialize;

use crate::{
    endpoints::{
        Endpoint,
        channel::messages::{allowed_mentions::AllowedMentions, create_message::CreateMessageBody},
    },
    request::Request,
};

#[derive(Builder, Clone, Debug)]
pub struct ScheduleMessage {
    pub channel_id: Id<ChannelMarker>,
    // TODO: Check whether the request body is really this, as it's not documented :(
    pub message: CreateMessageBody,
}

#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ScheduledMessageStatus {
    Pending,
    Invalid,
    Scheduled,
    Sent,
    Failed,
    Cancelled,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ScheduleMessageResponsePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<MessageAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<MessageEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    /// A client-provided value for message deduplication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Nonce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<MessageSticker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_ids: Option<Id<StickerMarker>>,
    #[serde(default)]
    pub tts: bool,
    #[serde(skip_serializing_if = "Option::is_none", rename = "favorite_meme_id")]
    pub saved_media_id: Option<Id<GenericMarker>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ScheduleMessageResponse {
    pub id: Id<ScheduledMessageMarker>,
    pub channel_id: Id<ChannelMarker>,
    // TODO: This is documented as being an ISO8601 __UTC__ timestamp, might wanna check if the format of
    // that is the same or not ...
    pub scheduled_at: Timestamp<Iso8601>,
    pub scheduled_local_at: Timestamp<Iso8601>,
    /// The IANA timezone identifier used for scheduling.
    pub timezone: String,
    pub status: ScheduledMessageStatus,
    /// A human-readable reason for the current status, if applicable.
    pub status_reason: Option<String>,
    pub payload: Message,
}

impl Endpoint for ScheduleMessage {
    type Response = ScheduleMessageResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.message).unwrap())
            .path(format!("/channels/{}/messages/schedule", self.channel_id))
            .build()
    }
}
