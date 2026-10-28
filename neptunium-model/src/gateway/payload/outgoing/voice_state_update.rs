use either::Either;

use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct VoiceStateUpdateOutgoing {
    /// The guild with the voice channel, where `Some(None)` selects the DM and group DM call context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Option<Id<GuildMarker>>>,
    /// The channel to join or move to, where `Some(None)` leaves the membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Option<Id<ChannelMarker>>>,
    /// The existing voice connection this update applies to.
    /// A `channel_id` with no `connection_id` opens a new connection and one with it updates or moves that connection.
    ///
    /// Leaving a guild with `channel_id: Some(None)` needs a `connection_id`, or the update is refused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    // TODO: Technically the string "true" is accepted too, which is dumb as hell.
    /// Whether the client has muted its microphone (default `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_mute: Option<bool>,
    /// Whether the client has deafened its output (default `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_deaf: Option<bool>,
    /// Whether the client publishes camera video (default `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_video: Option<bool>,
    /// Whether this connection advertises a screenshare track (default `false`)-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_stream: Option<bool>,
    /// Whether this is a mobile voice client (default `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mobile: Option<bool>,
    /// The stream keys this connection watches, where `None` keeps the list and `Some(None)` clears it.
    ///
    /// Each entry is a stream key matching this update’s scope, guild and channel, or the update is refused.
    /// An entry with no connection is also refused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_stream_keys: Option<Option<Vec<String>>>,
    /// The client latitude, used to pick a voice region.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "either::serde_untagged_optional"
    )]
    pub latitude: Option<Either<i64, String>>,
    /// The client longitude, used to pick a voice region.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "either::serde_untagged_optional"
    )]
    pub longitude: Option<Either<i64, String>>,
    /// Whether the client agrees to peer-to-peer media for a new connection, or leaves the mesh on an existing one.
    ///
    /// Only `true` agrees. On an existing connection, `false` moves the whole channel or call to LiveKit. A bot session never agrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2p: Option<bool>,
}
