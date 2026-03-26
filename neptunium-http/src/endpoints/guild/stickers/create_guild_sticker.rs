use bon::Builder;
use neptunium_model::{
    guild::properties::GuildSticker,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct CreateGuildStickerBody {
    #[builder(into)]
    pub name: String,
    /// Base64-encoded image data.
    #[builder(into)]
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,
    /// Up to 10 tags for autocomplete/suggestions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub tags: Option<Vec<String>>,
}

#[derive(Builder, Clone, Debug)]
pub struct CreateGuildSticker {
    pub guild_id: Id<GuildMarker>,
    pub body: CreateGuildStickerBody,
}

impl Endpoint for CreateGuildSticker {
    type Response = GuildSticker;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/stickers", self.guild_id))
            .build()
    }
}
