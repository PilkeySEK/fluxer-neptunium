use bon::Builder;
use neptunium_model::{
    guild::GuildResponse,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetGuildInformation {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for GetGuildInformation {
    type Response = GuildResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}", self.guild_id))
            .build()
    }
}
