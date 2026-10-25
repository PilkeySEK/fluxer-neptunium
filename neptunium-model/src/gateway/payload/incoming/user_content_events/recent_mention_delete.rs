use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::MessageMarker};

/// A recent mention was deleted from the user's mention list.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct RecentMentionDelete {
    pub message_id: Id<MessageMarker>,
}
