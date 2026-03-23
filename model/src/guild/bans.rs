use serde::Deserialize;

use crate::{
    id::{Id, marker::UserMarker},
    time::timestamp::{Timestamp, representations::Iso8601},
    user::UserPartial,
};

#[derive(Deserialize, Clone, Debug)]
pub struct GuildBanListEntry {
    // TODO: Check whether this is actually UserPartial
    pub user: UserPartial,
    pub moderator_id: Id<UserMarker>,
    pub banned_at: Timestamp<Iso8601>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// `None` if the ban is permanent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp<Iso8601>>,
}
