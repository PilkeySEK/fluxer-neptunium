use bon::Builder;
use neptunium_model::invites::InviteWithMetadata;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct GetInviteInformation {
    /// The invite code.
    #[builder(into)]
    pub code: String,
}

impl Endpoint for GetInviteInformation {
    type Response = InviteWithMetadata;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/invites/{}", self.code))
            .build()
    }
}
