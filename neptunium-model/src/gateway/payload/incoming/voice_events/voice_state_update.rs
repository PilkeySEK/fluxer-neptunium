use serde::Deserialize;

use crate::{
    guild::member::GuildMember,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
};

#[expect(clippy::struct_excessive_bools)]
#[derive(Deserialize, Clone, Debug)]
pub struct VoiceStateUpdate {
    /// Only present for guild voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Channel ID the user is in (`None` if disconnected).
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
    /// Voice state version number, incremented on each update.
    pub version: u64,
}
