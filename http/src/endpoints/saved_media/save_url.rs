use bon::Builder;
use neptunium_model::user::settings::FavoriteMeme;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

/// Save something to saved media from a provided URL.
#[derive(Builder, Serialize, Clone, Debug)]
pub struct SaveUrl {
    /// The display name.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klipy_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenor_slug_id: Option<String>,
}

impl Endpoint for SaveUrl {
    type Response = FavoriteMeme;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/memes".to_owned())
            .build()
    }
}
