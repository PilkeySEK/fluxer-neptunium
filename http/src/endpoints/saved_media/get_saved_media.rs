use bon::Builder;
use neptunium_model::user::settings::FavoriteMeme;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct GetSavedMedia {
    pub saved_media_id: String,
}

impl Endpoint for GetSavedMedia {
    type Response = FavoriteMeme;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/users/@me/memes/{}", self.saved_media_id))
            .build()
    }
}
