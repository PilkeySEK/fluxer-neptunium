use std::collections::HashMap;

use bon::Builder;
use neptunium_model::{
    channel::message::Message,
    id::{Id, marker::ChannelMarker},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

// https://docs.fluxer.app/api-reference/users/preload-messages-for-channels
/// Preloads and caches messages for multiple channels to improve performance when opening those channels.
#[derive(Builder, Serialize, Clone, Debug)]
pub struct PreloadMessagesForChannels {
    pub channels: Vec<Id<ChannelMarker>>,
}

impl Endpoint for PreloadMessagesForChannels {
    type Response = HashMap<Id<ChannelMarker>, Message>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/preload-messages".to_owned())
            .build()
    }
}
