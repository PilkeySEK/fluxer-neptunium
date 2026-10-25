use bon::Builder;
use neptunium_model::id::{Id, marker::GuildMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

/// Clears all hoist position assignments for roles in the guild.
#[derive(Builder, Clone, Debug)]
pub struct ResetGuildRoleHoistPositions {
    pub guild_id: Id<GuildMarker>,
    #[builder(into)]
    pub audit_log_reason: Option<String>,
}

impl Endpoint for ResetGuildRoleHoistPositions {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .maybe_audit_log_reason(self.audit_log_reason)
            .path(format!("/guilds/{}/roles/hoist-positions", self.guild_id))
            .build()
    }
}
