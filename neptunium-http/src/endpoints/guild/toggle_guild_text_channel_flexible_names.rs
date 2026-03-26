use bon::Builder;
use neptunium_model::{
    guild::Guild,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ToggleGuildTextChannelFlexibleNames {
    pub guild_id: Id<GuildMarker>,
    pub enabled: bool,
}

impl Endpoint for ToggleGuildTextChannelFlexibleNames {
    type Response = Guild;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(
                json!({
                "enabled": self.enabled,
                })
                .to_string(),
            )
            .path(format!(
                "/guilds/{}/text-channel-flexible-names",
                self.guild_id
            ))
            .build()
    }
}
