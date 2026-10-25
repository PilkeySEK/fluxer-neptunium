use bon::Builder;
use neptunium_model::{
    guild::properties::GuildEmoji,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;
use serde_json::json;

use crate::{
    endpoints::{Endpoint, guild::emoji::GuildEmojiCreateData},
    request::Request,
};

#[derive(Builder, Clone, Debug)]
pub struct CreateGuildEmoji {
    pub guild_id: Id<GuildMarker>,
    pub emoji: GuildEmojiCreateData,
}

impl Endpoint for CreateGuildEmoji {
    type Response = GuildEmoji;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .body(
                json!({
                    "name": self.emoji.name,
                    "image": self.emoji.image,
                })
                .to_string(),
            )
            .method(Method::POST)
            .path(format!("/guilds/{}/emojis", self.guild_id))
            .build()
    }
}
