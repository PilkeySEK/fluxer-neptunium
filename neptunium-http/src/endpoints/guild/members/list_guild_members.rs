use std::collections::HashMap;

use bon::Builder;
use neptunium_model::{
    guild::member::GuildMember,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildMembers {
    pub guild_id: Id<GuildMarker>,
    /// 1-1000, maximum number of users to return. The default is 1.
    pub limit: Option<u16>,
    pub after: Option<Id<UserMarker>>,
}

impl Endpoint for ListGuildMembers {
    type Response = Vec<GuildMember>;

    fn into_request(self) -> crate::request::Request {
        let mut params = HashMap::new();
        if let Some(limit) = self.limit {
            params.insert("limit".to_owned(), limit.to_string());
        }
        if let Some(after) = self.after {
            params.insert("after".to_owned(), after.to_string());
        }

        Request::builder()
            .method(Method::GET)
            .params(params)
            .path(format!("/guilds/{}/members", self.guild_id))
            .build()
    }
}
