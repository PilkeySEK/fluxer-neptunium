use bon::Builder;
use neptunium_model::{
    guild::properties::GuildSticker,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildStickers {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for ListGuildStickers {
    type Response = Vec<GuildSticker>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/stickers", self.guild_id))
            .build()
    }
}
