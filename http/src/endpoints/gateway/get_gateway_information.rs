use neptunium_model::time::duration::{Duration, representation::Millis};
use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct GatewayInformationSessionStartLimit {
    /// Total number of session starts allowed.
    pub total: u64,
    /// Remaining number of session starts.
    pub remaining: u64,
    /// Duration until the limit resets.
    pub reset_after: Duration<Millis>,
    /// Maximum number of concurrent `IDENTIFY` requests.
    pub max_concurrency: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GatewayInformation {
    /// WebSocket URL to connect to the gateway.
    pub url: String,
    /// Recommended number of shards to use when connecting.
    pub shards: u64,
    /// Session start rate limit information.
    pub session_start_limit: GatewayInformationSessionStartLimit,
}

#[derive(Copy, Clone, Debug)]
pub struct GetGatewayInformation;

impl Endpoint for GetGatewayInformation {
    type Response = GatewayInformation;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/gateway/bot".to_owned())
            .build()
    }
}
