use bon::Builder;
use neptunium_model::{
    guild::permissions::GuildRole,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildRoles {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for ListGuildRoles {
    type Response = Vec<GuildRole>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/roles", self.guild_id))
            .build()
    }
}
