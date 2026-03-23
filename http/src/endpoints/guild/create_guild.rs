use bon::Builder;
use neptunium_model::guild::GuildResponse;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct CreateGuild {
    #[builder(into)]
    pub name: String,
    /// Base64-encoded image data for the guild icon.
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Whether to create the guild without default features.
    #[builder(default = false)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub empty_features: bool,
}

impl Endpoint for CreateGuild {
    type Response = GuildResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/guilds".to_owned())
            .build()
    }
}
