use serde::{Deserialize, Serialize};

use crate::{
    guild::member::GuildMember,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
    time::timestamp::{Timestamp, representations::UnixMillis},
};

/// Sent when a user starts typing in a channel, both in DMs and in guilds.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypingStart {
    pub channel_id: Id<ChannelMarker>,
    pub user_id: Id<UserMarker>,
    /// Timestamp of when the typing started.
    pub timestamp: Timestamp<UnixMillis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,
}
