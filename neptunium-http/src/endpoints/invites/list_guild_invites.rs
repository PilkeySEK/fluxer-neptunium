use bon::Builder;
use neptunium_model::{
    id::{Id, marker::GuildMarker},
    invites::InviteWithMetadata,
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildInvites {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for ListGuildInvites {
    type Response = Vec<InviteWithMetadata>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/invites", self.guild_id))
            .build()
    }
}
