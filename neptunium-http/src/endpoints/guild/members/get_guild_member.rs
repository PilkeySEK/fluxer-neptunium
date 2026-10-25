use bon::Builder;
use neptunium_model::{
    guild::member::GuildMember,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetGuildMember {
    pub guild_id: Id<GuildMarker>,
    pub user_id: Id<UserMarker>,
}

impl Endpoint for GetGuildMember {
    type Response = GuildMember;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!(
                "/guilds/{}/members/{}",
                self.guild_id, self.user_id
            ))
            .build()
    }
}
