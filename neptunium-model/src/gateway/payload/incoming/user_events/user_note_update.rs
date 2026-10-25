use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::UserMarker};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserNoteUpdate {
    /// User ID the note is about.
    pub id: Id<UserMarker>,
    /// The note content.
    pub note: String,
}
