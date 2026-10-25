use bon::Builder;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DeleteSavedMedia {
    pub saved_media_id: String,
}

impl Endpoint for DeleteSavedMedia {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!("/users/@me/memes/{}", self.saved_media_id))
            .build()
    }
}
