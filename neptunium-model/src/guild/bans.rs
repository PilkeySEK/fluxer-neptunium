use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::UserMarker},
    time::timestamp::{Timestamp, representations::Iso8601},
    user::PartialUser,
};

/// An active guild ban.
#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct GuildBan {
    pub user: PartialUser,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub moderator_id: Id<UserMarker>,
    pub banned_at: Timestamp<Iso8601>,
    /// `None` if the ban is permanent.
    pub expires_at: Option<Timestamp<Iso8601>>,
}
