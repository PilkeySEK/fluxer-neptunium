use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListRtcRegions {
    pub channel_id: Id<ChannelMarker>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ListRtcRegionsResponseEntry {
    pub id: String,
    /// The display name.
    pub name: String,
    pub emoji: String,
}

impl Endpoint for ListRtcRegions {
    type Response = Vec<ListRtcRegionsResponseEntry>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/channels/{}/rtc-regions", self.channel_id))
            .build()
    }
}
