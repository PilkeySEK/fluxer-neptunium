use bon::Builder;
use neptunium_model::{
    guild::permissions::{GuildRole, Permissions},
    id::{
        Id,
        marker::{GuildMarker, RoleMarker},
    },
    misc::HexColor,
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

/// On the everyone role, Fluxer ignores every field except `color`, `permissions`, `icon` and `unicode_emoji`.
#[derive(Builder, Serialize, Clone, Debug)]
pub struct ModifyGuildRoleBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<HexColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    // TODO: Do null and undefined have seperate behaviors here?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist_position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
    #[serde(flatten)]
    pub icon: Option<ModifyGuildRoleBodyRoleIcon>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModifyGuildRoleBodyRoleIcon {
    /// A base64-encoded PNG, JPEG, WebP or AVIF image.
    ///
    /// Set to `None` to unset.
    Icon(Option<String>),
    /// A single unicode emoji.
    ///
    /// Set to `None` to unset.
    UnicodeEmoji(Option<String>),
}

#[derive(Builder, Clone, Debug)]
pub struct ModifyGuildRole {
    pub guild_id: Id<GuildMarker>,
    pub role_id: Id<RoleMarker>,
    pub body: ModifyGuildRoleBody,
    #[builder(into)]
    pub audit_log_reason: Option<String>,
}

impl Endpoint for ModifyGuildRole {
    type Response = GuildRole;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .maybe_audit_log_reason(self.audit_log_reason)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/roles/{}", self.guild_id, self.role_id))
            .build()
    }
}
