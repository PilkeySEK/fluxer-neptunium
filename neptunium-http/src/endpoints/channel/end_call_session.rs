use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

/// Terminate an active voice call in a channel.
#[derive(Builder, Copy, Clone, Debug)]
pub struct EndCallSession {
    pub channel_id: Id<ChannelMarker>,
}

impl Endpoint for EndCallSession {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path(format!("/channels/{}/call/end", self.channel_id))
            .build()
    }
}
