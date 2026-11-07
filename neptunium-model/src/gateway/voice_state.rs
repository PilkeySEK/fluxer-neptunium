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
    pub guild_id: Option<Id<GuildMarker>>,
    /// `None` if disconnected.
    pub channel_id: Option<Id<ChannelMarker>>,
    pub user_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,
    pub mute: bool,
    pub deaf: bool,
    pub self_mute: bool,
    pub self_deaf: bool,
    #[serde(default)]
    pub self_video: bool,
    #[serde(default)]
    pub self_stream: bool,
    #[serde(default)]
    pub is_mobile: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_stream_keys: Option<Vec<String>>,
    /// The voice state version for ordering updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}
