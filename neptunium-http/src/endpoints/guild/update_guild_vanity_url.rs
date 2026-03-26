use bon::Builder;
use neptunium_model::id::{Id, marker::GuildMarker};
use reqwest::Method;
use serde::Deserialize;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct UpdateGuildVanityUrl {
    pub guild_id: Id<GuildMarker>,
    // TODO: Is this true?
    /// Set this to `None` to remove the vanity url.
    #[builder(into)]
    pub code: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateGuildVanityUrlResponse {
    pub code: String,
}

impl Endpoint for UpdateGuildVanityUrl {
    type Response = UpdateGuildVanityUrlResponse;

    fn into_request(self) -> crate::request::Request {
        let body = if let Some(code) = self.code {
            json!({
                "code": code,
            })
        } else {
            json!({
                "code": serde_json::Value::Null,
            })
        };

        Request::builder()
            .method(Method::PATCH)
            .body(body.to_string())
            .path(format!("/guilds/{}/vanity-url", self.guild_id))
            .build()
    }
}
