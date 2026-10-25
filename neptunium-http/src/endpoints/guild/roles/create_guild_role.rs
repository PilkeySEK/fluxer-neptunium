use bon::Builder;
use neptunium_model::{
    guild::permissions::{GuildRole, Permissions},
    id::{Id, marker::GuildMarker},
    misc::HexColor,
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug, Serialize)]
pub struct CreateGuildRoleBody {
    #[builder(into)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<HexColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
}

#[derive(Builder, Clone, Debug)]
pub struct CreateGuildRole {
    pub guild_id: Id<GuildMarker>,
    pub body: CreateGuildRoleBody,
    #[builder(into)]
    pub audit_log_reason: Option<String>,
}

impl Endpoint for CreateGuildRole {
    type Response = GuildRole;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .maybe_audit_log_reason(self.audit_log_reason)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/roles", self.guild_id))
            .build()
    }
}
