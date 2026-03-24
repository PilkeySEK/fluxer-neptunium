use bon::Builder;
use neptunium_model::{
    guild::properties::GuildEmoji,
    id::{Id, marker::GuildMarker},
    user::PartialUser,
};
use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Deserialize, Clone, Debug)]
pub struct ListGuildEmojisResponseEntry {
    #[serde(flatten)]
    pub emoji: GuildEmoji,
    pub user: PartialUser,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildEmojis {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for ListGuildEmojis {
    type Response = Vec<ListGuildEmojisResponseEntry>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/emojis", self.guild_id))
            .build()
    }
}
