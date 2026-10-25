use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, GuildMarker, MessageMarker},
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

/// Returns the public profile of the guild a copy of a published message came from.
#[derive(Builder, Copy, Clone, Debug)]
pub struct GetMessageCrosspostSource {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrosspostSource {
    pub guild: CrosspostSourceGuild,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrosspostSourceGuild {
    pub id: Id<GuildMarker>,
    pub name: String,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub description: Option<String>,
    pub features: Vec<CrosspostSourceGuildFeatureFlag>,
    pub approximate_member_count: Option<usize>,
    pub approximate_presence_count: Option<usize>,
    /// Is true when discovery is on for the instance, the guild is listed in discovery, and its invites are not disabled.
    pub discoverable: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CrosspostSourceGuildFeatureFlag {
    Verified,
    Partnered,
    Discoverable,
}

impl Endpoint for GetMessageCrosspostSource {
    type Response = CrosspostSource;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!(
                "/channels/{}/messages/{}/crosspost-source",
                self.channel_id, self.message_id
            ))
            .build()
    }
}
