use bon::Builder;
use neptunium_model::id::{Id, marker::UserMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct RemoveRelationship {
    pub user_id: Id<UserMarker>,
}

impl Endpoint for RemoveRelationship {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!("/users/@me/relationships/{}", self.user_id))
            .build()
    }
}
