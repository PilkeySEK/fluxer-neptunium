use bitflags::bitflags;
use bon::Builder;
use serde_repr::{Deserialize_repr, Serialize_repr};
use zeroize::Zeroizing;

use crate::{
    gateway::{intents::GatewayEventFlags, shard::ShardInfo},
    id::{
        Id,
        marker::{ApplicationMarker, EmojiMarker},
    },
};

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum ActivityType {
    Playing = 0,
    Streaming,
    Listening,
    Watching,
    Custom,
    Competing,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub struct Timestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
}

// TODO: Why is this here??
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Emoji {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<EmojiMarker>>,
    #[serde(default)]
    pub animated: bool,
}

impl std::fmt::Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.id.is_some() {
            f.write_str(":")?;
            f.write_str(&self.name)?;
            f.write_str(":")
        } else {
            f.write_str(&self.name)
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
pub enum StatusDisplayType {
    Name = 0,
    State,
    Details,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ActivityParty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `(current_size, max_size)`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<(i32, i32)>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ActivityAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_cover_image: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ActivitySecrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ActivityFlags: u64 {
        const INSTANCE = 1 << 0;
        const JOIN = 1 << 1;
        const SPECTATE = 1 << 2;
        const JOIN_REQUEST = 1 << 3;
        const SYNC = 1 << 4;
        const PLAY = 1 << 5;
        const PARTY_PRIVACY_FRIENDS = 1 << 6;
        const PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7;
        const EMBEDDED = 1 << 8;
    }
}

impl<'de> serde::Deserialize<'de> for ActivityFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl serde::Serialize for ActivityFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}

/// Bot users are only able to set `name`, `state`, `type`, and `url`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Builder)]
pub struct Activity {
    #[builder(into)]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: ActivityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[builder(default = {
        #[expect(clippy::cast_possible_truncation)]
        let value = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
        value
    })]
    pub created_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Timestamps>,
    /// Application ID for the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_display_type: Option<StatusDisplayType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ActivitySecrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ActivityFlags>,
    /// Max. 2 buttons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<ActivityButton>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UpdatePresence {
    /// Time when the client went idle, or `None` if the client is not idle
    pub since: Option<u64>,
    pub activities: Vec<Activity>,
    pub status: String,
    pub afk: bool,
}

// TODO: This has more (optional) fields when logging in as a user
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ConnectionProperties {
    /// The operating system, e.g. "linux".
    pub os: String,
    /// The library name.
    pub browser: String,
    /// The library name.
    pub device: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Identify {
    pub token: Zeroizing<String>,
    pub properties: ConnectionProperties,
    // Doesn't appear to exist - https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/fluxer_gateway/src/gateway/gateway_handler.erl#L466
    // pub compress: bool,
    // /// Value between 50 and 250.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub large_threshold: Option<u8>,
    // Also doesn't appear to exist, but I'll be keeping this for the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<ShardInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<UpdatePresence>,
    // TODO: Check whether this is actually `Intents`...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_events: Option<GatewayEventFlags>,
    // TODO: There might be a "flags" field, have to check...
}
