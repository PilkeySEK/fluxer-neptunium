use bon::Builder;
use serde::Serialize;

use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};

/// Sent to update the voice state, as well as joining and leaving voice.
#[expect(clippy::struct_excessive_bools)]
#[derive(Serialize, Clone, Debug, Builder)]
pub struct UpdateVoiceState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Set to `None` to disconnect, otherwise specify the ID of the voice channel.
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Set to `None` if no voice connection/session exists yet.
    #[builder(into)]
    pub connection_id: Option<String>,
    #[builder(default = false)]
    pub self_deaf: bool,
    #[builder(default = false)]
    pub self_mute: bool,
    #[builder(default = false)]
    pub self_video: bool,
    #[builder(default = false)]
    pub self_stream: bool,
    #[builder(default = false)]
    pub is_mobile: bool,
    #[builder(default = Vec::new())]
    pub viewer_stream_keys: Vec<String>,
    // TODO: Are these optional or required?
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}
