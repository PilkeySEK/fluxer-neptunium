use bon::Builder;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DeleteInvite {
    /// The invite code.
    #[builder(into)]
    pub code: String,
}

impl Endpoint for DeleteInvite {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!("/invites/{}", self.code))
            .build()
    }
}
