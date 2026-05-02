use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

/// Currently this is only for purging the personal notes.
#[derive(Copy, Clone, Debug, Builder)]
pub struct PurgeChannelMessages {
    pub channel_id: Id<ChannelMarker>,
}

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PurgeChannelMessagesResponse {
    pub deleted_count: usize,
}

impl Endpoint for PurgeChannelMessages {
    type Response = PurgeChannelMessagesResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body("{}".to_owned())
            .path(format!("/channels/{}/messages/purge", self.channel_id))
            .build()
    }
}
