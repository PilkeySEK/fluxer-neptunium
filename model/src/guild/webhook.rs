use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zeroize::Zeroizing;

use crate::{
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker, WebhookMarker},
    },
    misc::{HexColor32, ImageHash},
    user::flags::PublicUserFlags,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebhookUser {
    pub id: Id<UserMarker>,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<ImageHash>,
    pub avatar_color: Option<HexColor32>,
    pub flags: PublicUserFlags,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Webhook {
    pub id: Id<WebhookMarker>,
    pub guild_id: Id<GuildMarker>,
    pub channel_id: Id<ChannelMarker>,
    /// The display name.
    pub name: String,
    pub token: Zeroizing<String>,
    pub user: WebhookUser,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2,
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
    pub avatar_hash: Option<ImageHash>,
}
