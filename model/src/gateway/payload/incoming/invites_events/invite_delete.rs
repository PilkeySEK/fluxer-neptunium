use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InviteDelete {
    /// The invite code that was deleted.
    pub code: String,
    /// Present for guild and group DM invites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Only present for guild invites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
}
