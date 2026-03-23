use bon::Builder;
use neptunium_model::{
    id::{Id, marker::GuildMarker},
    user::auth::SudoVerification,
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DeleteGuild {
    pub guild_id: Id<GuildMarker>,
    pub auth: SudoVerification,
}

impl Endpoint for DeleteGuild {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.auth).unwrap())
            .path(format!("/guilds/{}/delete", self.guild_id))
            .build()
    }
}
