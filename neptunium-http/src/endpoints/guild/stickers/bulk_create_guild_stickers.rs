use bon::Builder;
use neptunium_model::{
    guild::properties::GuildSticker,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    endpoints::{Endpoint, guild::stickers::create_guild_sticker::CreateGuildStickerBody},
    request::Request,
};

#[derive(Builder, Clone, Debug)]
pub struct BulkCreateGuildStickers {
    pub guild_id: Id<GuildMarker>,
    /// 1-50 stickers.
    pub body: Vec<CreateGuildStickerBody>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BulkCreateGuildStickersFailureResponseEntry {
    pub name: String,
    pub error: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BulkCreateGuildStickersResponse {
    /// Successfully created stickers.
    pub success: Vec<GuildSticker>,
    /// Stickers that failed to create.
    pub failed: Vec<BulkCreateGuildStickersFailureResponseEntry>,
}

impl Endpoint for BulkCreateGuildStickers {
    type Response = BulkCreateGuildStickersResponse;

    fn into_request(self) -> crate::request::Request {
        #[derive(Serialize)]
        struct BulkCreateGuildStickersBody {
            stickers: Vec<CreateGuildStickerBody>,
        }

        let body = BulkCreateGuildStickersBody {
            stickers: self.body,
        };

        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&body).unwrap())
            .path(format!("/guilds/{}/stickers/bulk", self.guild_id))
            .build()
    }
}
