use std::collections::HashMap;

use bon::Builder;
use neptunium_model::{
    channel::message::Message,
    id::{
        Id,
        marker::{ChannelMarker, GenericMarker},
    },
    time::{
        OffsetDateTime,
        timestamp::{Timestamp, representations::UnixMillis},
    },
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug, Default)]
pub struct ListChannelMessagesParams {
    /// Number of messages to return. 1-100, defaults to 50.
    pub limit: Option<u8>,
    pub before: Option<Timestamp<UnixMillis>>,
    pub after: Option<Timestamp<UnixMillis>>,
    pub around: Option<Timestamp<UnixMillis>>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListChannelMessages {
    pub channel_id: Id<ChannelMarker>,
    pub params: ListChannelMessagesParams,
}

impl Endpoint for ListChannelMessages {
    type Response = Vec<Message>;

    fn into_request(self) -> crate::request::Request {
        let mut params = HashMap::<String, String>::new();
        if let Some(limit) = self.params.limit {
            params.insert(String::from("limit"), limit.to_string());
        }
        if let Some(before) = self.params.before {
            params.insert(
                String::from("before"),
                Id::<GenericMarker>::from(OffsetDateTime::from(before)).to_string(),
            );
        }
        if let Some(after) = self.params.after {
            params.insert(
                String::from("after"),
                Id::<GenericMarker>::from(OffsetDateTime::from(after)).to_string(),
            );
        }
        if let Some(around) = self.params.around {
            params.insert(
                String::from("around"),
                Id::<GenericMarker>::from(OffsetDateTime::from(around)).to_string(),
            );
        }

        Request::builder()
            .params(params)
            .method(Method::GET)
            .path(format!("/channels/{}/messages", self.channel_id))
            .build()
    }
}
