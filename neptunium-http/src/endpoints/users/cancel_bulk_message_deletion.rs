use reqwest::Method;
use serde::Deserialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct CancelBulkMessageDeletion;

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct CancelBulkMessageDeletionResponse {
    /// This is always `true`.
    pub success: bool,
}

impl Endpoint for CancelBulkMessageDeletion {
    type Response = CancelBulkMessageDeletionResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path("/users/@me/messages/delete".to_owned())
            .build()
    }
}
