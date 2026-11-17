use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::UserMarker};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageCall {
    // ISO8601
    #[serde(skip_serializing_if = "Option::is_none")]
    ended_timestamp: Option<jiff::Timestamp>,
    participants: Vec<Id<UserMarker>>,
}
