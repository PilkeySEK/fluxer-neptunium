use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct PinDirectMessageChannel {
    pub channel_id: Id<ChannelMarker>,
}

impl Endpoint for PinDirectMessageChannel {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PUT)
            .path(format!("/users/@me/channels/{}/pin", self.channel_id))
            .build()
    }
}
