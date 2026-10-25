use serde::{Deserialize, Serialize};

use crate::{
    guild::member::GuildMember,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
};

#[expect(clippy::struct_excessive_bools)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VoiceState {
    /// `None` if in a DM call.
    guild_id: Option<Id<GuildMarker>>,
    /// `None` if disconnected.
    channel_id: Option<Id<ChannelMarker>>,
    user_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<GuildMember>,
    mute: bool,
    deaf: bool,
    self_mute: bool,
    self_deaf: bool,
    #[serde(default)]
    self_video: bool,
    #[serde(default)]
    self_stream: bool,
    #[serde(default)]
    is_mobile: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    viewer_stream_keys: Option<Vec<String>>,
    /// The voice state version for ordering updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<u64>,
}
