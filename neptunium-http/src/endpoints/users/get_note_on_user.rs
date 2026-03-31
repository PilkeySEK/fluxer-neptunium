use bon::Builder;
use neptunium_model::id::{Id, marker::UserMarker};
use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetUserNote {
    pub user_id: Id<UserMarker>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GetUserNoteResponse {
    pub note: String,
}

impl Endpoint for GetUserNote {
    type Response = GetUserNoteResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/users/@me/notes/{}", self.user_id))
            .build()
    }
}
