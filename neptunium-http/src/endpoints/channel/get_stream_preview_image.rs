use bon::Builder;
use reqwest::Method;

use crate::{
    endpoints::{Endpoint, ResponseBody},
    request::Request,
};

#[derive(Builder, Clone, Debug)]
pub struct GetStreamPreviewImage {
    pub stream_key: String,
}

/// The bytes of a JPEG image.
#[derive(Clone, Debug)]
pub struct StreamPreviewImageResponse(pub Vec<u8>);

impl ResponseBody for StreamPreviewImageResponse {
    fn deserialize(
        bytes: Vec<u8>,
    ) -> Result<Self, Box<crate::endpoints::ExecuteEndpointRequestError>> {
        Ok(Self(bytes))
    }
    fn serialize(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl Endpoint for GetStreamPreviewImage {
    type Response = StreamPreviewImageResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path(format!("/streams/{}/preview", self.stream_key))
            .build()
    }
}
