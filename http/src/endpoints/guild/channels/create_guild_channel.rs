use bon::Builder;
use neptunium_model::{
    channel::Channel,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    endpoints::{Endpoint, guild::channels::ChannelRequestBase},
    request::Request,
};

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u16)]
pub enum GuildChannelCreateType {
    Text = 0,
    Voice = 2,
    Category = 4,
    Link = 998,
}

#[derive(Serialize, Builder, Clone, Debug)]
pub struct GuildChannelCreateRequest {
    #[serde(flatten)]
    pub base: ChannelRequestBase,
    #[builder(default = false)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub nsfw: bool,
    /// The channel name.
    #[builder(into)]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: GuildChannelCreateType,
}

#[derive(Builder, Clone, Debug)]
pub struct CreateGuildChannel {
    pub guild_id: Id<GuildMarker>,
    pub body: GuildChannelCreateRequest,
}

impl Endpoint for CreateGuildChannel {
    type Response = Channel;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/channels", self.guild_id))
            .build()
    }
}
