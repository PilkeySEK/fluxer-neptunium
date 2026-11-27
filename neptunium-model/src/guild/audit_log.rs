use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    id::{
        Id,
        marker::{AuditLogEntryMarker, ChannelMarker, GenericMarker, MessageMarker, UserMarker},
    },
    time::duration::{Duration, Seconds},
};

// mod action;
// pub use action::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GuildAuditLogEntry {
    pub id: Id<AuditLogEntryMarker>,
    pub action_type: AuditLogActionType,
    /// The user that did the action.
    #[serde(rename = "user_id")]
    pub user: AuditLogEntryUser,
    pub target_id: Option<Id<GenericMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AuditLogOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<AuditLogChange>>,
}

// TODO: Make this depend on the action type
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AuditLogOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_member_days: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_message_seconds: Option<Duration<Seconds>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<GenericMarker>>,
    // TODO: What type is this?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<Duration<Seconds>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_removed: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Id<MessageMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<usize>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AuditLogChange {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<serde_json::Value>,
}

#[derive(
    Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
#[repr(u16)]
pub enum AuditLogActionType {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelOverwriteCreate = 13,
    ChannelOverwriteUpdate = 14,
    ChannelOverwriteDelete = 15,
    MemberKick = 20,
    MemberPrune = 21,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    MemberRoleUpdate = 25,
    MemberMove = 26,
    MemberDisconnect = 27,
    BotAdd = 28,
    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,
    InviteCreate = 40,
    InviteUpdate = 41,
    InviteDelete = 42,
    WebhookCreate = 50,
    WebhookUpdate = 51,
    WebhookDelete = 52,
    EmojiCreate = 60,
    EmojiUpdate = 61,
    EmojiDelete = 62,
    MessageDelete = 72,
    MessageBulkDelete = 73,
    MessagePin = 74,
    MessageUnpin = 75,
    StickerCreate = 90,
    StickerUpdate = 91,
    StickerDelete = 92,
    ThreadCreate = 110,
    ThreadUpdate = 111,
    ThreadDelete = 112,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditLogEntryUser {
    System,
    User(Id<UserMarker>),
}

impl serde::Serialize for AuditLogEntryUser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::System => Id::<UserMarker>::new(0).serialize(serializer),
            Self::User(id) => id.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for AuditLogEntryUser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = Id::deserialize(deserializer)?;
        Ok(if id.is_system_user() {
            Self::System
        } else {
            Self::User(id)
        })
    }
}
