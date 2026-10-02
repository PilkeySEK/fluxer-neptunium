use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zeroize::Zeroizing;

use crate::{
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, WebhookMarker},
    },
    user::PartialUser,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Webhook {
    pub id: Id<WebhookMarker>,
    pub guild_id: Id<GuildMarker>,
    pub channel_id: Id<ChannelMarker>,
    /// The display name.
    pub name: String,
    /// Not present for a channel follower webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<Zeroizing<String>>,
    #[serde(rename = "user")]
    pub creator: PartialUser,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "type")]
    pub r#type: WebhookType,
    /// resent only on a channel follower webhook, and only while the account that created it can view the announcement channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<WebhookSource>,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WebhookSource {
    pub guild: WebhookSourceGuild,
    pub channel: WebhookSourceChannel,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WebhookSourceGuild {
    pub id: Id<GuildMarker>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WebhookSourceChannel {
    pub id: Id<ChannelMarker>,
    pub name: String,
}

/// A webhook as represented in the audit log.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuditLogWebhook {
    pub id: Id<WebhookMarker>,
    #[serde(rename = "type")]
    pub r#type: WebhookType,
    pub guild_id: Id<GuildMarker>,
    pub channel_id: Id<ChannelMarker>,
    /// The display name.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_hash: Option<String>,
}
