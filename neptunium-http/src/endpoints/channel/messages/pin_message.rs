use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct PinMessage {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
}

impl Endpoint for PinMessage {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PUT)
            .path(format!(
                "/channels/{}/pins/{}",
                self.channel_id, self.message_id
            ))
            .build()
    }
}
