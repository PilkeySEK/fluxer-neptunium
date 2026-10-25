use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

// Deleting a message does not support a reason currently
#[derive(Builder, Copy, Clone, Debug)]
pub struct DeleteMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
}

impl Endpoint for DeleteMessage {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!(
                "/channels/{}/messages/{}",
                self.channel_id, self.message_id
            ))
            .build()
    }
}
