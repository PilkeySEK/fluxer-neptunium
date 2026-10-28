mod call_create;
mod call_delete;
mod call_update;

pub use call_create::*;
pub use call_delete::*;
pub use call_update::*;

use crate::{
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, SoundMarker, UserMarker},
    },
    time::duration::{Duration, Millis},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct VoiceSignalIncoming {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    pub channel_id: Id<ChannelMarker>,
    /// The `connection_id` of the sender.
    pub from: String,
    pub user_id: Id<UserMarker>,
    // TODO: See type from source code.
    /// The signaling message exactly as the sender sent it.
    /// The type is undocumented.
    pub data: serde_json::Value,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EntranceSoundPlay {
    pub user_id: Id<UserMarker>,
    pub channel_id: Id<ChannelMarker>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub sound_id: Id<SoundMarker>,
    /// Content hash of the sound file.
    pub hash: String,
    /// URL to fetch the sound from.
    pub url: String,
    pub duration_ms: Duration<Millis>,
    /// MIME type of the sound file.
    pub content_type: String,
}
