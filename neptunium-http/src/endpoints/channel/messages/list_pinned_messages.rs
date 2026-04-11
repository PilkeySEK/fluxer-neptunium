use std::collections::HashMap;

use bon::Builder;
use neptunium_model::{
    channel::message::Message,
    id::{Id, marker::ChannelMarker},
    time::{
        UtcDateTime,
        timestamp::{Timestamp, representations::UnixMillis},
    },
};
use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug, Default)]
pub struct ListPinnedMessagesParams {
    /// 1-50.
    pub limit: Option<u8>,
    // TODO: Check whether this is UnixMillis or not
    pub before: Option<Timestamp<UnixMillis>>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListPinnedMessages {
    pub channel_id: Id<ChannelMarker>,
    pub params: ListPinnedMessagesParams,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ListPinnedMessagesResponse {
    /// Whether more pins can be fetched with pagination.
    pub has_more: bool,
    pub items: Vec<Message>,
}

impl Endpoint for ListPinnedMessages {
    type Response = ListPinnedMessagesResponse;

    fn into_request(self) -> crate::request::Request {
        let mut params = HashMap::new();
        if let Some(limit) = self.params.limit {
            params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(before) = self.params.before {
            let timestamp = UtcDateTime::from(before);
            let timestamp =
                (timestamp.unix_timestamp() * 1000) + i64::from(timestamp.millisecond());
            params.insert("before".to_string(), timestamp.to_string());
        }
        Request::builder()
            .method(Method::GET)
            .params(params)
            .path(format!("/channels/{}/messages/pins", self.channel_id))
            .build()
    }
}
