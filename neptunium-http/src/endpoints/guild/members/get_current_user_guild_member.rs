use bon::Builder;
use neptunium_model::{
    guild::member::GuildMember,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetCurrentUserGuildMember {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for GetCurrentUserGuildMember {
    type Response = GuildMember;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/members/@me", self.guild_id))
            .build()
    }
}
