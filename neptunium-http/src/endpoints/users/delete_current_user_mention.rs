use bon::Builder;
use neptunium_model::id::{Id, marker::MessageMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct DeleteMention {
    pub message_id: Id<MessageMarker>,
}

impl Endpoint for DeleteMention {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!("/users/@me/mentions/{}", self.message_id))
            .build()
    }
}
