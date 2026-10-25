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

#[derive(Builder, Copy, Clone, Debug)]
pub struct FetchMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
}

impl Endpoint for FetchMessage {
    type Response = Message;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!(
                "/channels/{}/messages/{}",
                self.channel_id, self.message_id
            ))
            .build()
    }
}
