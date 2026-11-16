use bon::Builder;
use neptunium_model::{
    channel::message::Message,
    id::{Id, marker::ChannelMarker},
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
pub struct ListPinnedMessagesParams {
    /// 1-50.
    pub limit: Option<u8>,
    // ISO8601
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<jiff::Timestamp>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListPinnedMessages {
    pub channel_id: Id<ChannelMarker>,
    pub params: ListPinnedMessagesParams,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ListPinnedMessagesResponse {
    /// Whether more pins can be fetched with pagination.
    pub has_more: bool,
    pub items: Vec<Message>,
}

impl Endpoint for ListPinnedMessages {
    type Response = ListPinnedMessagesResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .params(serde_qs::to_string(&self.params).unwrap())
            .path(format!("/channels/{}/messages/pins", self.channel_id))
            .build()
    }
}
