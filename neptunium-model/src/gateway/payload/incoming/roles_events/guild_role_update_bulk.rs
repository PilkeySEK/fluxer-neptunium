use serde::{Deserialize, Serialize};

use crate::{
    guild::permissions::GuildRole,
    id::{Id, marker::GuildMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildRoleUpdateBulk {
    pub roles: Vec<GuildRole>,
    pub guild_id: Id<GuildMarker>,
}
