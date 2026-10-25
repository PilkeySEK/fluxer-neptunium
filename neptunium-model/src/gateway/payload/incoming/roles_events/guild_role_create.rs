use serde::{Deserialize, Serialize};

use crate::{
    guild::permissions::GuildRole,
    id::{Id, marker::GuildMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildRoleCreate {
    pub role: GuildRole,
    pub guild_id: Id<GuildMarker>,
}
