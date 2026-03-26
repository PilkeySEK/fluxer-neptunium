use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{GuildMarker, RoleMarker},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Copy, Clone, Debug)]
pub struct UpdateGuildRolePositionsEntry {
    pub id: Id<RoleMarker>,
    pub position: u64,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateGuildRolePositions {
    pub guild_id: Id<GuildMarker>,
    pub body: Vec<UpdateGuildRolePositionsEntry>,
}

impl Endpoint for UpdateGuildRolePositions {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/roles", self.guild_id))
            .build()
    }
}
