use bon::Builder;
use neptunium_model::{
    id::{
        Id,
        marker::{AttachmentMarker, ChannelMarker, MessageMarker},
    },
    user::settings::FavoriteMeme,
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Builder, Clone, Debug)]
pub struct SaveMessageAttachmentBody {
    /// The display name.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// ID of the message attachment to save.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<Id<AttachmentMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_index: Option<u64>,
}

/// Saves a message attachment to the saved media.
#[derive(Builder, Clone, Debug)]
pub struct SaveMessageAttachment {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub body: SaveMessageAttachmentBody,
}

impl Endpoint for SaveMessageAttachment {
    type Response = FavoriteMeme;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!(
                "/channels/{}/messages/{}/memes",
                self.channel_id, self.message_id
            ))
            .build()
    }
}
