use bon::Builder;
use neptunium_model::user::settings::FavoriteMeme;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Clone, Debug, Builder)]
pub struct UpdateSavedMediaBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateSavedMedia {
    pub saved_media_id: String,
    pub body: UpdateSavedMediaBody,
}

impl Endpoint for UpdateSavedMedia {
    type Response = FavoriteMeme;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/users/@me/memes/{}", self.saved_media_id))
            .build()
    }
}
