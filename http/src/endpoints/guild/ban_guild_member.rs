use bon::Builder;
use neptunium_model::{
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
    time::duration::{Duration, representation::Seconds},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct BanGuildMemberBody {
    /// 0-7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_message_days: Option<u8>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 0 means permanent.
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "ban_duration_seconds"
    )]
    pub ban_duration: Option<Duration<Seconds>>,
}

#[derive(Builder, Clone, Debug)]
pub struct BanGuildMember {
    pub guild_id: Id<GuildMarker>,
    pub user_id: Id<UserMarker>,
    pub body: BanGuildMemberBody,
}

impl Endpoint for BanGuildMember {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PUT)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/bans/{}", self.guild_id, self.user_id))
            .build()
    }
}
