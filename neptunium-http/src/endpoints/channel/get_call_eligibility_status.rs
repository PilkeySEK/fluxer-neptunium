use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Deserialize, Copy, Clone, Debug, Serialize)]
pub struct CallEligibilityStatus {
    pub ringable: bool,
    pub silent: bool,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetCallEligibilityStatus {
    pub channel_id: Id<ChannelMarker>,
}

impl Endpoint for GetCallEligibilityStatus {
    type Response = CallEligibilityStatus;
    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/channels/{}/call", self.channel_id))
            .build()
    }
}
