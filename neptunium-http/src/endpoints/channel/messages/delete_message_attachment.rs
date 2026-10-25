use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{AttachmentMarker, ChannelMarker, MessageMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DeleteMessageAttachment {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub attachment_id: Id<AttachmentMarker>,
}

impl Endpoint for DeleteMessageAttachment {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!(
                "/channels/{}/messages/{}/attachments/{}",
                self.channel_id, self.message_id, self.attachment_id
            ))
            .build()
    }
}
