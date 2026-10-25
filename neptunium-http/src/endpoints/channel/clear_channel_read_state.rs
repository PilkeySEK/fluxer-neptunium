use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

/// Clears all read state and acknowledgement records for a channel, marking all messages as unread.
#[derive(Builder, Copy, Clone, Debug)]
pub struct ClearChannelReadState {
    pub channel_id: Id<ChannelMarker>,
}

impl Endpoint for ClearChannelReadState {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!("/channels/{}/messages/ack", self.channel_id))
            .build()
    }
}
