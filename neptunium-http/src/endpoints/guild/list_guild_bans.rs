use bon::Builder;
use neptunium_model::{
    guild::bans::GuildBanListEntry,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildBans {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for ListGuildBans {
    type Response = Vec<GuildBanListEntry>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/bans", self.guild_id))
            .build()
    }
}
