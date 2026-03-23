use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Builder, Copy, Clone, Debug)]
pub struct UpdateGuildChannelPositionsEntry {
    pub id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    /// New parent category id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    /// ID of the sibling channel that should directly precede this channel after reordering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preceding_sibling_id: Option<Id<ChannelMarker>>,
    /// Whether to sync permissions with the new parent.
    #[serde(rename = "lock_permissions", skip_serializing_if = "Option::is_none")]
    pub sync_permissions: Option<bool>,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateGuildChannelPositions {
    pub guild_id: Id<GuildMarker>,
    pub body: Vec<UpdateGuildChannelPositionsEntry>,
}

impl Endpoint for UpdateGuildChannelPositions {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/channels", self.guild_id))
            .build()
    }
}
