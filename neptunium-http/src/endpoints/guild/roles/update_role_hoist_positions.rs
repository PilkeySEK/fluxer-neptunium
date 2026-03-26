use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{GuildMarker, RoleMarker},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Copy, Clone, Debug)]
pub struct UpdateGuildRoleHoistPositionsEntry {
    pub id: Id<RoleMarker>,
    pub hoist_position: u64,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateGuildRoleHoistPositions {
    pub guild_id: Id<GuildMarker>,
    pub body: Vec<UpdateGuildRoleHoistPositionsEntry>,
}

impl Endpoint for UpdateGuildRoleHoistPositions {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/roles/hoist-positions", self.guild_id))
            .build()
    }
}
