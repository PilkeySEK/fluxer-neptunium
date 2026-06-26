use neptunium_model::time::timestamp::{Timestamp, representations::Iso8601};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HandoffInitiate;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HandoffInitiateResponse {
    /// Handoff code to share with the recieving device.
    pub code: String,
    pub expires_at: Timestamp<Iso8601>,
}

impl Endpoint for HandoffInitiate {
    type Response = HandoffInitiateResponse;
    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path("/auth/handoff/initiate".to_owned())
            .build()
    }
}
