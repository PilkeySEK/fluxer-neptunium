mod user_note_update;

pub use user_note_update::*;

use crate::user::UserConnection;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserConnectionsUpdate {
    pub connections: Vec<UserConnection>,
}
