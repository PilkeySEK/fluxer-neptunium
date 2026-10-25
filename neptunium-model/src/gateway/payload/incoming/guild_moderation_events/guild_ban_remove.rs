use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::GuildMarker},
    user::UserIDObject,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildBanRemove {
    pub guild_id: Id<GuildMarker>,
    // https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/api/src/guild/services/GuildModerationService.tsx#L202
    // While the documentation says that this is a PartialUser, the event really only contains the ID.
    pub user: UserIDObject,
}
