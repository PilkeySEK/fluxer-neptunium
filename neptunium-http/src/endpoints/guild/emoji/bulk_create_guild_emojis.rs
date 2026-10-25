use bon::Builder;
use neptunium_model::{
    guild::properties::GuildEmoji,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    endpoints::{
        Endpoint,
        guild::emoji::{EmojiCreateFailure, GuildEmojiCreateData},
    },
    request::Request,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BulkCreateGuildEmojisResponse {
    /// Successfully created emojis.
    pub success: Vec<GuildEmoji>,
    /// Emojis that failed to create.
    pub failed: Vec<EmojiCreateFailure>,
}

#[derive(Builder, Clone, Debug)]
pub struct BulkCreateGuildEmojis {
    pub guild_id: Id<GuildMarker>,
    pub emojis: Vec<GuildEmojiCreateData>,
}

impl Endpoint for BulkCreateGuildEmojis {
    type Response = BulkCreateGuildEmojisResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .body(
                json!({
                    "emojis": self.emojis,
                })
                .to_string(),
            )
            .method(Method::POST)
            .path(format!("/guilds/{}/emojis/bulk", self.guild_id))
            .build()
    }
}
