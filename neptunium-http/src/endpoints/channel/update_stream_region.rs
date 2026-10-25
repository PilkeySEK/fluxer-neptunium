use bon::Builder;
use neptunium_model::channel::VoiceRegion;
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct UpdateStreamRegion {
    pub stream_key: String,
    pub region: VoiceRegion,
}

impl Endpoint for UpdateStreamRegion {
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
            .path(format!("/streams/{}/stream", self.stream_key))
            .build()
    }
}
