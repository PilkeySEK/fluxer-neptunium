use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::GuildMarker},
    user::PartialUser,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildBanRemove {
    pub guild_id: Id<GuildMarker>,
    pub user: PartialUser,
}
