use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

/// Marks a message as read and records acknowledgement state.
#[derive(Builder, Copy, Clone, Debug)]
pub struct AcknowledgeMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    /// Update the mention count.
    pub mention_count: Option<u64>,
    /// Whether this is a manual acknowledgement.
    pub manual: Option<bool>,
}

impl Endpoint for AcknowledgeMessage {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        #[derive(Serialize)]
        struct AcknowledgeMessageBody {
            #[serde(skip_serializing_if = "Option::is_none")]
            mention_count: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            manual: Option<bool>,
        }

        let body = AcknowledgeMessageBody {
            mention_count: self.mention_count,
            manual: self.manual,
        };

        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&body).unwrap())
            .path(format!(
                "/channels/{}/messages/{}/ack",
                self.channel_id, self.message_id
            ))
            .build()
    }
}
