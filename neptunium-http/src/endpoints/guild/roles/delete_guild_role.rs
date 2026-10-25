use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{GuildMarker, RoleMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DeleteGuildRole {
    pub guild_id: Id<GuildMarker>,
    pub role_id: Id<RoleMarker>,
    #[builder(into)]
    pub audit_log_reason: Option<String>,
}

impl Endpoint for DeleteGuildRole {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .maybe_audit_log_reason(self.audit_log_reason)
            .path(format!("/guilds/{}/roles/{}", self.guild_id, self.role_id))
            .build()
    }
}
