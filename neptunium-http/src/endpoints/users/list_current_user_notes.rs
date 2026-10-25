use std::collections::HashMap;

use neptunium_model::id::{Id, marker::UserMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct ListCurrentUserNotes;

impl Endpoint for ListCurrentUserNotes {
    type Response = HashMap<Id<UserMarker>, String>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/notes".to_owned())
            .build()
    }
}
