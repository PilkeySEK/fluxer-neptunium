use bon::Builder;
use neptunium_model::{
    guild::bans::GuildBan,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildBans {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for ListGuildBans {
    type Response = Vec<GuildBan>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/bans", self.guild_id))
            .build()
    }
}

#[derive(Builder, Copy, Clone, Debug, PartialEq, Eq)]
pub struct GetGuildBan {
    pub guild_id: Id<GuildMarker>,
    pub user_id: Id<UserMarker>,
}

impl Endpoint for GetGuildBan {
    type Response = GuildBan;

    fn into_request(self) -> Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/bans/{}", self.guild_id, self.user_id))
            .build()
    }
}
