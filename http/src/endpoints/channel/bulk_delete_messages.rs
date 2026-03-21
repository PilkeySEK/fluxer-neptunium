use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct BulkDeleteMessages {
    pub channel_id: Id<ChannelMarker>,
    /// IDs of messages to delete.
    pub messages: Vec<Id<MessageMarker>>,
}

impl Endpoint for BulkDeleteMessages {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(
                json!({
                    "message_ids": self.messages,
                })
                .to_string(),
            )
            .path(format!(
                "/channels/{}/messages/bulk-delete",
                self.channel_id
            ))
            .build()
    }
}
