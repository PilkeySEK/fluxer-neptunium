use bon::Builder;
use neptunium_model::{
    channel::message::Message,
    id::{Id, marker::MessageMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ListCurrentUserMentionsParams {
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

#[derive(Builder, Clone, Debug)]
pub struct ListCurrentUserMentions {
    pub params: ListCurrentUserMentionsParams,
}

impl Endpoint for ListCurrentUserMentions {
    type Response = Vec<Message>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .params(serde_qs::to_string(&self.params).unwrap())
            .path("/users/@me/mentions".to_owned())
            .build()
    }
}
