use std::collections::HashMap;

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

#[derive(Builder, Default, Copy, Clone, Debug)]
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
        let mut params = HashMap::new();

        if let Some(limit) = self.params.limit {
            params.insert("limit".to_owned(), limit.to_string());
        }
        if let Some(before) = self.params.before {
            params.insert("before".to_owned(), before.to_string());
        }
        if let Some(after) = self.params.after {
            params.insert("after".to_owned(), after.to_string());
        }
        if let Some(user_id) = self.params.user_id {
            params.insert("user_id".to_owned(), user_id.to_string());
        }
        if let Some(action_type) = self.params.action_type {
            params.insert("action_type".to_owned(), (action_type as u8).to_string());
        }

        Request::builder()
            .method(Method::GET)
            .params(params)
            .path(format!("/guilds/{}/audit-logs", self.guild_id))
            .build()
    }
}
