use bon::Builder;
use neptunium_model::id::{Id, marker::GuildMarker};
use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetGuildVanityUrl {
    pub guild_id: Id<GuildMarker>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GetGuildVanityUrlResponse {
    pub uses: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl Endpoint for GetGuildVanityUrl {
    type Response = GetGuildVanityUrlResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/vanity-url", self.guild_id))
            .build()
    }
}
