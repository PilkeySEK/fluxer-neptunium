use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RequestChannelMemberCounts {
    /// The ID of the guild whose channels are queried.
    pub guild_id: Id<GuildMarker>,
    /// The channel IDs to query, deduplicated and truncated to `25`.
    pub channel_ids: Vec<Id<ChannelMarker>>,
    /// The ID of the single channel to query.
    ///
    /// Used only when `channel_ids` is `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// A value of `1`-`64` bytes echoed in the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}
