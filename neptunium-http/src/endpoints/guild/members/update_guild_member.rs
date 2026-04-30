use bon::Builder;
use neptunium_model::{
    guild::member::GuildMember,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
    },
    time::timestamp::{Timestamp, representations::Iso8601},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Clone, Debug, Builder)]
pub struct UpdateGuildMemberBody {
    #[builder(into)]
    pub nick: Option<String>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Id<RoleMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
    // TODO: Is this true?
    /// Set this to `Some(None)` to remove the user's timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<Option<Timestamp<Iso8601>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub timeout_reason: Option<String>,
    // TODO: How to disconnect a member? Set this to null? Check that
    /// The voice channel to move the member to.
    #[serde(skip_serializing_if = "Option::is_none", rename = "channel_id")]
    pub voice_channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connection_id")]
    pub voice_connection_id: Option<String>,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateGuildMember {
    pub guild_id: Id<GuildMarker>,
    pub user_id: Id<UserMarker>,
    pub body: UpdateGuildMemberBody,
    /// The reason for the audit log.
    #[builder(into)]
    pub reason: Option<String>,
}

impl Endpoint for UpdateGuildMember {
    type Response = GuildMember;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!(
                "/guilds/{}/members/{}",
                self.guild_id, self.user_id
            ))
            .maybe_audit_log_reason(self.reason)
            .build()
    }
}
