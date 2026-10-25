use bon::Builder;
use neptunium_model::user::relationship::Relationship;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct SendFriendRequestByTag {
    #[builder(into)]
    pub username: String,
    #[builder(into)]
    pub discriminator: String,
}

impl Endpoint for SendFriendRequestByTag {
    type Response = Relationship;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/relationships".to_owned())
            .build()
    }
}
