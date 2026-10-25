use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::MessageMarker};

/// A saved message was deleted from the user's saved messages.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct SavedMessageDelete {
    pub message_id: Id<MessageMarker>,
}
