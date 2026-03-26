use bon::Builder;
use neptunium_model::id::{Id, marker::GuildMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

/// Clears all hoist position assignments for roles in the guild.
#[derive(Builder, Copy, Clone, Debug)]
pub struct ResetGuildRoleHoistPositions {
    pub guild_id: Id<GuildMarker>,
}

impl Endpoint for ResetGuildRoleHoistPositions {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!("/guilds/{}/roles/hoist-positions", self.guild_id))
            .build()
    }
}
