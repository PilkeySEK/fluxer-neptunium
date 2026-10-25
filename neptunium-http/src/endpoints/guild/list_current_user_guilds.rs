use std::collections::HashMap;

use bon::Builder;
use neptunium_model::{
    guild::Guild,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

// TODO: What is bearer token?

#[derive(Builder, Copy, Clone, Debug)]
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
        let mut params = HashMap::new();

        if let Some(before) = self.params.before {
            params.insert("before".to_owned(), before.to_string());
        }
        if let Some(after) = self.params.after {
            params.insert("after".to_owned(), after.to_string());
        }
        if let Some(limit) = self.params.limit {
            params.insert("limit".to_owned(), limit.to_string());
        }
        if let Some(with_counts) = self.params.with_counts {
            params.insert("with_counts".to_owned(), with_counts.to_string());
        }

        Request::builder()
            .method(Method::GET)
            .params(params)
            .path("/users/@me/guilds".to_owned())
            .build()
    }
}
