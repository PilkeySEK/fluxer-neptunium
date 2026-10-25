use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Copy, Clone, Debug)]
pub struct SaveMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
}

impl Endpoint for SaveMessage {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/saved-messages".to_owned())
            .build()
    }
}
