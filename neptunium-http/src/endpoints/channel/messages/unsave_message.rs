use bon::Builder;
use neptunium_model::id::{Id, marker::MessageMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct UnsaveMessage {
    pub message_id: Id<MessageMarker>,
}

impl Endpoint for UnsaveMessage {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!("/users/@me/saved-messages/{}", self.message_id))
            .build()
    }
}
