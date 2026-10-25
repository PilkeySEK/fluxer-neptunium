use neptunium_model::gateway::payload::incoming::UserPrivateResponse;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct GetCurrentUserProfile;

impl Endpoint for GetCurrentUserProfile {
    type Response = UserPrivateResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me".to_owned())
            .build()
    }
}
