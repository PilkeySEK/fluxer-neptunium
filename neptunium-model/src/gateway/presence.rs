use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    id::{
        Id,
        marker::{EmojiMarker, GuildMarker},
    },
    time::timestamp::{Timestamp, representations::Iso8601},
    user::PartialUser,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PresenceStatus {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "dnd")]
    DoNotDisturb,
    #[serde(rename = "invisible")]
    Invisible,
    #[serde(rename = "offline")]
    Offline,
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug)]
pub struct CustomStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub emoji_id: Option<Id<EmojiMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub emoji_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub emoji_animated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub expires_at: Option<Timestamp<Iso8601>>,
}

/// Represents a user's presence (online status and activity).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Presence {
    pub user: PartialUser,
    pub status: PresenceStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_status: Option<CustomStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
}
