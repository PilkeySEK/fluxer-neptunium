use bon::Builder;
use neptunium_model::{
    channel::message::Message,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

/// Publishes a message in an announcement channel to every channel that follows it.
#[derive(Builder, Copy, Clone, Debug)]
pub struct CrosspostMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
}

impl Endpoint for CrosspostMessage {
    type Response = Message;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path(format!(
                "/channels/{}/messages/{}/crosspost",
                self.channel_id, self.message_id
            ))
            .build()
    }
}
