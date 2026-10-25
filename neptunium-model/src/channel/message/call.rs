use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::UserMarker},
    time::timestamp::{Timestamp, representations::Iso8601},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    ended_timestamp: Option<Timestamp<Iso8601>>,
    participants: Vec<Id<UserMarker>>,
}
