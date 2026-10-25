use serde::{Deserialize, Serialize};

use crate::{
    guild::member::GuildMember,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
};

// TODO: Probably make a seperate type "GuildVoiceState" which is flattened into this struct so that type stuff is nicer.
#[expect(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoiceStateUpdate {
    /// Only present for guild voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    // TODO: Check what "null" means here, undefined or null?
    /// Channel ID the user is in (null if disconnected).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    pub user_id: Id<UserMarker>,
    pub connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Only present for guild voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_video: bool,
    pub self_stream: bool,
    pub is_mobile: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_stream_keys: Option<Vec<String>>,
    // TODO: What is this actually?
    /// Voice state version number, incremented on each update.
    pub version: u64,
}
