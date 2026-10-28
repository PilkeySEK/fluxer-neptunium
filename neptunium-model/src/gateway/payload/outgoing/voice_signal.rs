use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct VoiceSignalOutgoing {
    /// The guild with the voice channel, where `None` selects the DM and group DM call context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// The voice channel or call channel both connections are in.
    pub channel_id: Id<ChannelMarker>,
    /// The `connection_id` of the voice state the signal is for.
    pub to: String,
    /// The signaling message, which Fluxer relays unread.
    pub data: serde_json::Value,
}
