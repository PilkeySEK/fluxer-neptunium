use bon::Builder;
use neptunium_model::{
    guild::member::GuildMember, id::{Id, marker::{GuildMarker, UserMarker}},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub struct ListGuildMembersParams {
    /// 1-1000, maximum number of users to return. The default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<Id<UserMarker>>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildMembers {
    pub guild_id: Id<GuildMarker>,
    pub params: ListGuildMembersParams,
}

impl Endpoint for ListGuildMembers {
    type Response = Vec<GuildMember>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .params(serde_qs::to_string(&self.params).unwrap())
            .path(format!("/guilds/{}/members", self.guild_id))
            .build()
    }
}
