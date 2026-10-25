use bon::Builder;
use neptunium_model::{
    channel::VoiceRegion,
    id::{Id, marker::ChannelMarker},
};
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

/// Change the voice region of an active call.
#[derive(Builder, Clone, Debug)]
pub struct UpdateCallRegion {
    pub channel_id: Id<ChannelMarker>,
    pub region: VoiceRegion,
}

impl Endpoint for UpdateCallRegion {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(
                json!({
                    "region": self.region,
                })
                .to_string(),
            )
            .path(format!("/channels/{}/call", self.channel_id))
            .build()
    }
}
