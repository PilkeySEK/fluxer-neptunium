use std::collections::HashMap;

use bon::Builder;
use neptunium_model::{
    channel::message::Message,
    id::{Id, marker::MessageMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct ListCurrentUserMentions {
    /// 1-100, default 25.
    pub limit: Option<u8>,
    /// Whether to include role mentions.
    pub roles: Option<bool>,
    /// Whether to include @everyone mentions.
    pub everyone: Option<bool>,
    /// Whether to include mentions in guilds.
    pub guilds: Option<bool>,
    // TODO: Is this MessageMarker?
    pub before: Option<Id<MessageMarker>>,
}

impl Endpoint for ListCurrentUserMentions {
    type Response = Vec<Message>;

    fn into_request(self) -> crate::request::Request {
        let mut params = HashMap::new();

        if let Some(limit) = self.limit {
            params.insert("limit".to_owned(), limit.to_string());
        }
        if let Some(roles) = self.roles {
            params.insert("roles".to_owned(), roles.to_string());
        }
        if let Some(everyone) = self.everyone {
            params.insert("everyone".to_owned(), everyone.to_string());
        }
        if let Some(guilds) = self.guilds {
            params.insert("guilds".to_owned(), guilds.to_string());
        }
        if let Some(before) = self.before {
            params.insert("before".to_owned(), before.to_string());
        }

        Request::builder()
            .method(Method::GET)
            .params(params)
            .path("/users/@me/mentions".to_owned())
            .build()
    }
}
