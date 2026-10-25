use serde::{Deserialize, Serialize};

/// Authentication session has changed. Sent when the user's token is rotated due to a
/// security-sensitive operation like password change.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthSessionChange {
    pub old_auth_session_id_hash: String,
    pub new_auth_session_id_hash: String,
    pub new_token: String,
}
