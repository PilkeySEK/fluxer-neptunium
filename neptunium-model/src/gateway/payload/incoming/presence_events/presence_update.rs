use serde::{Deserialize, Serialize};

use crate::{gateway::presence::CustomStatus, user::PartialUser};

/// User presence was updated. Sent when a friend’s or group DM member’s status changes.
///
/// Dispatched to users who are subscribed to the target user’s presence (friends, group DM members).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PresenceUpdateIncoming {
    // TOOD: Check whether this is actually UserPartial or something else
    /// Normalized user object.
    pub user: PartialUser,
    /// User's current status.
    pub status: String,
    pub mobile: bool,
    pub afk: bool,
    pub custom_status: Option<CustomStatus>,
}
