use serde::{Deserialize, Serialize};

use crate::{gateway::presence::CustomStatus, user::UserIDObject};

/// User presence was updated. Sent when a friend’s or group DM member’s status changes.
///
/// Dispatched to users who are subscribed to the target user’s presence (friends, group DM members).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PresenceUpdateIncoming {
    pub user: UserIDObject,
    /// User's current status.
    pub status: String,
    pub mobile: bool,
    pub afk: bool,
    pub custom_status: Option<CustomStatus>,
}
