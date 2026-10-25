use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct AcknowledgeNewPinNotifications {
    pub channel_id: Id<ChannelMarker>,
}

impl Endpoint for AcknowledgeNewPinNotifications {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path(format!("/channels/{}/pins/ack", self.channel_id))
            .build()
    }
}
