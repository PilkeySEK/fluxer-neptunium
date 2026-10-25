use bon::Builder;
use neptunium_model::{
    id::{Id, marker::UserMarker},
    user::PartialUser,
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetUserById {
    pub user_id: Id<UserMarker>,
}

impl Endpoint for GetUserById {
    type Response = PartialUser;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/users/{}", self.user_id))
            .build()
    }
}
