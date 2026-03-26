use bon::Builder;
use neptunium_model::id::{Id, marker::GuildMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct LeaveGuild {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for LeaveGuild {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!("/users/@me/guilds/{}", self.guild_id))
            .build()
    }
}
