use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::GuildMarker},
    user::UserPartial,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildBanAdd {
    pub guild_id: Id<GuildMarker>,
    pub user: UserPartial,
}
