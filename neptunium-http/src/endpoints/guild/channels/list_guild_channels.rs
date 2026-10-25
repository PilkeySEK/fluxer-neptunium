use bon::Builder;
use neptunium_model::{
    channel::Channel,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildChannels {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for ListGuildChannels {
    type Response = Vec<Channel>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/channels", self.guild_id))
            .build()
    }
}
