use bon::Builder;
use neptunium_model::{
    guild::properties::GuildSticker,
    id::{
        Id,
        marker::{GuildMarker, StickerMarker},
    },
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct UpdateGuildStickerBody {
    #[builder(into)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,
    /// Up to 10 tags for autocomplete/suggestions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub tags: Option<Vec<String>>,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateGuildSticker {
    pub guild_id: Id<GuildMarker>,
    pub sticker_id: Id<StickerMarker>,
    pub body: UpdateGuildStickerBody,
}

impl Endpoint for UpdateGuildSticker {
    type Response = GuildSticker;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!(
                "/guilds/{}/stickers/{}",
                self.guild_id, self.sticker_id
            ))
            .build()
    }
}
