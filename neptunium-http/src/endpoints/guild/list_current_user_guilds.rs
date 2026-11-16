use bon::Builder;
use neptunium_model::{
    guild::Guild,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub struct ListCurrentUserGuildsParams {
    pub before: Option<Id<GuildMarker>>,
    pub after: Option<Id<GuildMarker>>,
    /// 1-200.
    pub limit: Option<usize>,
    /// Include approximate member and presence counts.
    pub with_counts: Option<bool>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListCurrentUserGuilds {
    pub params: ListCurrentUserGuildsParams,
}

impl Endpoint for ListCurrentUserGuilds {
    type Response = Vec<Guild>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .params(serde_qs::to_string(&self.params).unwrap())
            .path("/users/@me/guilds".to_owned())
            .build()
    }
}
