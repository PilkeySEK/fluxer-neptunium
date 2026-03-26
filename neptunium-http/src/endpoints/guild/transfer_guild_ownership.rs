use bon::Builder;
use neptunium_model::{
    guild::Guild,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};
use reqwest::Method;
use serde_json::json;
use zeroize::Zeroizing;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct TransferGuildOwnership {
    pub guild_id: Id<GuildMarker>,
    pub new_owner_id: Id<UserMarker>,
    #[builder(into)]
    pub password: Option<Zeroizing<String>>,
}

impl Endpoint for TransferGuildOwnership {
    type Response = Guild;

    fn into_request(self) -> crate::request::Request {
        let body = if let Some(password) = self.password {
            json!({
                "password": password,
            })
        } else {
            json!({})
        };

        Request::builder()
            .method(Method::POST)
            .body(body.to_string())
            .path(format!("/guilds/{}/transfer-ownership", self.guild_id))
            .build()
    }
}
