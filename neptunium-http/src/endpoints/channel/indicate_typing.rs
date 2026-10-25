use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct IndicateTyping {
    pub channel_id: Id<ChannelMarker>,
}

impl Endpoint for IndicateTyping {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path(format!("/channels/{}/typing", self.channel_id))
            .build()
    }
}
