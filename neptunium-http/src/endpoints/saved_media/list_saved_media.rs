use neptunium_model::user::settings::SavedMedia;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct ListSavedMedia;

impl Endpoint for ListSavedMedia {
    type Response = Vec<SavedMedia>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/memes".to_owned())
            .build()
    }
}
