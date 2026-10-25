use bon::Builder;
use neptunium_model::{
    guild::properties::GuildEmoji,
    id::{
        Id,
        marker::{EmojiMarker, GuildMarker},
    },
};
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct UpdateGuildEmoji {
    pub guild_id: Id<GuildMarker>,
    pub emoji_id: Id<EmojiMarker>,
    pub new_name: String,
}

impl Endpoint for UpdateGuildEmoji {
    type Response = GuildEmoji;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .body(
                json!({
                    "name": self.new_name,
                })
                .to_string(),
            )
            .method(Method::PATCH)
            .path(format!(
                "/guilds/{}/emojis/{}",
                self.guild_id, self.emoji_id
            ))
            .build()
    }
}
