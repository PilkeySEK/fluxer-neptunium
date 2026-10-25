use bon::Builder;
use neptunium_model::{
    id::{Id, marker::ChannelMarker},
    invites::InviteWithMetadata,
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListChannelInvites {
    pub channel_id: Id<ChannelMarker>,
}

impl Endpoint for ListChannelInvites {
    type Response = Vec<InviteWithMetadata>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/channels/{}/invites", self.channel_id))
            .build()
    }
}
