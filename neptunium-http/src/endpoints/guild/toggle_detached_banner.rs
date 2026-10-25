use bon::Builder;
use neptunium_model::{
    guild::Guild,
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct ToggleDetachedBanner {
    pub guild_id: Id<GuildMarker>,
    pub enabled: bool,
    #[builder(into)]
    pub audit_log_reason: Option<String>,
}

impl Endpoint for ToggleDetachedBanner {
    type Response = Guild;

    fn into_request(self) -> crate::request::Request {
        let body = json!({
            "enabled": self.enabled,
        })
        .to_string();

        Request::builder()
            .method(Method::PATCH)
            .maybe_audit_log_reason(self.audit_log_reason)
            .body(body)
            .path(format!("/guilds/{}/detached-banner", self.guild_id))
            .build()
    }
}
