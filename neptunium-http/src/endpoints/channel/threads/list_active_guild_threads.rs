use neptunium_model::{
    channel::thread::{ThreadChannel, ThreadMember},
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(bon::Builder, Copy, Clone, Debug, PartialEq, Eq)]
pub struct ListActiveGuildThreads {
    pub guild_id: Id<GuildMarker>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ActiveGuildThreads {
    pub threads: Vec<ThreadChannel>,
    /// A thread member object for each returned thread the bot joined.
    pub members: Vec<ThreadMember>,
}

impl Endpoint for ListActiveGuildThreads {
    type Response = ActiveGuildThreads;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/guilds/{}/threads/active", self.guild_id))
            .build()
    }
}
