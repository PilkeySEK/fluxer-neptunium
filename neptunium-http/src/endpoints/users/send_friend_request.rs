use bon::Builder;
use neptunium_model::{
    id::{Id, marker::UserMarker},
    user::relationship::Relationship,
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct SendFriendRequest {
    pub user_id: Id<UserMarker>,
}

impl Endpoint for SendFriendRequest {
    type Response = Relationship;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path(format!("/users/@me/relationships/{}", self.user_id))
            .build()
    }
}
