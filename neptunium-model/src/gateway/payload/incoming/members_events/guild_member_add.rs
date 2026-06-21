use serde::{Deserialize, Serialize};

use crate::{
    guild::member::GuildMember,
    id::{Id, marker::GuildMarker},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildMemberAdd {
    pub guild_id: Id<GuildMarker>,
    #[serde(flatten)]
    pub member: GuildMember,
}
