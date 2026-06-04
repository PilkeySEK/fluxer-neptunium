use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{GuildMarker, RoleMarker},
};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GuildRoleDelete {
    pub role_id: Id<RoleMarker>,
    pub guild_id: Id<GuildMarker>,
}
