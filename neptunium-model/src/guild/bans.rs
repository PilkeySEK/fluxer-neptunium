use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::UserMarker},
    user::PartialUser,
};

/// An active guild ban.
#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct GuildBan {
    pub user: PartialUser,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub moderator_id: Id<UserMarker>,
    // ISO8601
    pub banned_at: jiff::Timestamp,
    // ISO8601
    /// `None` if the ban is permanent.
    pub expires_at: Option<jiff::Timestamp>,
}
