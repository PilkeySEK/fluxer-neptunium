use neptunium_model::user::relationship::Relationship;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct ListRelationships;

impl Endpoint for ListRelationships {
    type Response = Vec<Relationship>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/relationships".to_owned())
            .build()
    }
}
