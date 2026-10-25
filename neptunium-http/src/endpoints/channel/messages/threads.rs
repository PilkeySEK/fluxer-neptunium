use neptunium_model::{
    channel::thread::{ThreadAutoArchiveDuration, ThreadChannel},
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct StartThreadFromMessageData {
    /// 1-100 characters.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    /// Seconds a user has to wait before sending another message (0-21600).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
}

/// Creates a public thread from an existing message in a text channel. The thread shares the ID of the message, so a message can start one thread.
#[derive(Serialize, Deserialize, bon::Builder, Clone, Debug, PartialEq, Eq)]
pub struct StartThreadFromMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub data: StartThreadFromMessageData,
}

impl Endpoint for StartThreadFromMessage {
    type Response = ThreadChannel;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.data).unwrap())
            .path(format!(
                "/channels/{}/messages/{}/threads",
                self.channel_id, self.message_id
            ))
            .build()
    }
}
