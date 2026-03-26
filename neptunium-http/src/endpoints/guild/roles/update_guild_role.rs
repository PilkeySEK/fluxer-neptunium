use bon::Builder;
use neptunium_model::{
    guild::permissions::{GuildRole, Permissions},
    id::{
        Id,
        marker::{GuildMarker, RoleMarker},
    },
    misc::HexColor32,
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct UpdateGuildRoleBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<HexColor32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    // TODO: Do null and undefined have seperate behaviors here?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist_position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateGuildRole {
    pub guild_id: Id<GuildMarker>,
    pub role_id: Id<RoleMarker>,
    pub body: UpdateGuildRoleBody,
}

impl Endpoint for UpdateGuildRole {
    type Response = GuildRole;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/roles/{}", self.guild_id, self.role_id))
            .build()
    }
}
