use bon::Builder;
use neptunium_model::{
    channel::thread::ThreadChannel,
    guild::{
        audit_log::{AuditLogActionType, GuildAuditLogEntry},
        webhook::AuditLogWebhook,
    },
    id::{
        Id,
        marker::{AuditLogEntryMarker, GuildMarker, UserMarker},
    },
    user::PartialUser,
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, serde::Serialize, serde::Deserialize, Default, Copy, Clone, Debug)]
pub struct ListGuildAuditLogsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<Id<AuditLogEntryMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<Id<AuditLogEntryMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<AuditLogActionType>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListGuildAuditLogs {
    pub guild_id: Id<GuildMarker>,
    pub params: ListGuildAuditLogsParams,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GuildAuditLogs {
    pub audit_log_entries: Vec<GuildAuditLogEntry>,
    pub users: Vec<PartialUser>,
    pub webhooks: Vec<AuditLogWebhook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<ThreadChannel>>,
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
