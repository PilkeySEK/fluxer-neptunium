use bon::Builder;
use neptunium_model::{
    channel::Channel,
    id::{Id, marker::ChannelMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetChannel {
    pub channel_id: Id<ChannelMarker>,
}

impl Endpoint for GetChannel {
    type Response = Channel;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/channels/{}", self.channel_id))
            .build()
    }
}
