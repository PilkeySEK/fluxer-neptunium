use bon::Builder;
use neptunium_model::invites::Invite;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct AcceptInvite {
    /// The invite code.
    #[builder(into)]
    pub code: String,
}

impl Endpoint for AcceptInvite {
    type Response = Invite;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path(format!("/invites/{}", self.code))
            .build()
    }
}
