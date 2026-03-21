use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    guild::permissions::Permissions,
    id::{
        Id,
        marker::{ChannelMarker, GenericMarker, GuildMarker, MessageMarker, UserMarker},
    },
    time::timestamp::{Timestamp, representations::Iso8601},
    user::UserPartial,
};

pub mod message;

#[derive(Clone, Debug)]
pub enum VoiceRegion {
    Automatic,
    Fixed(String),
}

impl<'de> Deserialize<'de> for VoiceRegion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: Make this more efficient by not deserializing into a value first
        let value = Value::deserialize(deserializer)?;

        match value {
            Value::Null => Ok(Self::Automatic),
            Value::String(s) => Ok(Self::Fixed(s)),
            // TODO: Maybe use unexpected instead but that is annoying to do
            _ => Err(serde::de::Error::custom("expected either a string or null")),
        }
    }
}

impl Serialize for VoiceRegion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Automatic => Value::Null.serialize(serializer),
            Self::Fixed(s) => s.serialize(serializer),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelPartialRecipient {
    pub username: String,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum PermissionOverwriteEntity {
    Role = 0,
    Member = 1,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PermissionOverwrite {
    pub allow: Permissions,
    pub deny: Permissions,
    /// Can be either a role ID or a user ID
    pub id: Id<GenericMarker>,
    /// The type of entity this overwrite applies to.
    #[serde(rename = "type")]
    pub r#type: PermissionOverwriteEntity,
}

// https://github.com/fluxerapp/fluxer/blob/03813bbe17db008452f0f1be3090a7d2970a5447/packages/constants/src/ChannelConstants.tsx#L22
#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u16)]
pub enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildLink = 998,
    DmPersonalNotes = 999,
}

// TODO: Make this an enum instead maybe.
// Maybe find out why twilight does it in this way too (and not an enum)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    /// The bitrate of the voice channel in bits per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// The icon hash of the channel (for group DMs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<Id<MessageMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp<Iso8601>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Custom nicknames for users in this channel (for group DMs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nicks: Option<HashMap<Id<UserMarker>, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// The ID of the owner of the channel (for group DMs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<UserPartial>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<VoiceRegion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename = "type")]
    pub r#type: ChannelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelPartial {
    pub id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<ChannelPartialRecipient>>,
    // TODO: figure out what this number means (its the type of the channel)
    #[serde(rename = "type")]
    pub r#type: i32,
}
