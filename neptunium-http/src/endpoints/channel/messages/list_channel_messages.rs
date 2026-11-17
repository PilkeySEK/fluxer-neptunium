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

#[derive(Builder, serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
pub struct ListChannelMessagesParams {
    /// Number of messages to return. 1-100, defaults to 50.
    pub limit: Option<u8>,
    pub before: Option<Id<MessageMarker>>,
    pub after: Option<Id<MessageMarker>>,
    pub around: Option<Id<MessageMarker>>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListChannelMessages {
    pub channel_id: Id<ChannelMarker>,
    pub params: ListChannelMessagesParams,
}

impl Endpoint for ListChannelMessages {
    type Response = Vec<Message>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .params(serde_qs::to_string(&self.params).unwrap())
            .method(Method::GET)
            .path(format!("/channels/{}/messages", self.channel_id))
            .build()
    }
}
