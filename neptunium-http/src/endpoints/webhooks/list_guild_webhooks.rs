use bon::Builder;
use neptunium_model::{
    guild::webhook::Webhook,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildWebhooks {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for ListGuildWebhooks {
    type Response = Vec<Webhook>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/webhooks", self.guild_id))
            .build()
    }
}
