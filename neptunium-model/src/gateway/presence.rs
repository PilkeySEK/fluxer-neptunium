use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::EmojiMarker},
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
    #[builder(into)]
    pub text: Option<String>,
    #[builder(into)]
    pub emoji_id: Option<Id<EmojiMarker>>,
    #[builder(into)]
    pub emoji_name: Option<String>,
    #[builder(into)]
    pub expires_at: Option<Timestamp<Iso8601>>,
}

/// Represents a user's presence (online status and activity).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Presence {
    user: PartialUser,
    status: PresenceStatus,
    mobile: bool,
    afk: bool,
    custom_status: Option<CustomStatus>,
}
