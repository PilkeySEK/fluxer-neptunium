use bon::Builder;
use neptunium_model::{
    guild::audit_log::{GuildAuditLogs, event_type::AuditLogActionType},
    id::{
        Id,
        marker::{AuditLogEntryMarker, GuildMarker, UserMarker},
    },
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, serde::Serialize, serde::Deserialize, Default, Copy, Clone, Debug)]
pub struct ListGuildAuditLogsParams {
    pub limit: Option<u32>,
    pub before: Option<Id<AuditLogEntryMarker>>,
    pub after: Option<Id<AuditLogEntryMarker>>,
    pub user_id: Option<Id<UserMarker>>,
    pub action_type: Option<AuditLogActionType>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildAuditLogs {
    pub guild_id: Id<GuildMarker>,
    pub params: ListGuildAuditLogsParams,
}

impl Endpoint for ListGuildAuditLogs {
    type Response = GuildAuditLogs;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .params(serde_qs::to_string(&self.params).unwrap())
            .path(format!("/guilds/{}/audit-logs", self.guild_id))
            .build()
    }
}
