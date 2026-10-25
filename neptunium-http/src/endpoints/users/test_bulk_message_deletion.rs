use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct TestBulkMessageDeletion;

impl Endpoint for TestBulkMessageDeletion {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path("/users/@me/messages/delete/test".to_owned())
            .build()
    }
}
