mod presence_update;

pub use presence_update::*;

use crate::{
    gateway::presence::Presence,
    id::{Id, marker::GuildMarker},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PresenceUpdateBulk {
    pub guild_id: Id<GuildMarker>,
    pub presences: Vec<Presence>,
}
