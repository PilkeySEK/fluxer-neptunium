use bon::Builder;
use neptunium_model::invites::InviteWithMetadata;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct ListPackInvites {
    #[builder(into)]
    pub pack_id: String,
}

impl Endpoint for ListPackInvites {
    type Response = Vec<InviteWithMetadata>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/packs/{}/invites", self.pack_id))
            .build()
    }
}
