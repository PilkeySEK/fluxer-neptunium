use reqwest::Method;

use crate::{common_types::SuccessResponse, endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct CancelBulkMessageDeletion;

impl Endpoint for CancelBulkMessageDeletion {
    type Response = SuccessResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path("/users/@me/messages/delete".to_owned())
            .build()
    }
}
