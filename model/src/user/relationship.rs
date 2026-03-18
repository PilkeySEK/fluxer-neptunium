use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    id::{Id, marker::RelationshipMarker},
    time::timestamp::{Timestamp, representations::Iso8601},
    user::UserPartial,
};

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RelationshipType {
    Friend = 1,
    Blocked = 2,
    PendingIncomingFriendRequest = 3,
    PendingOutgoingFriendRequest = 4,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Relationship {
    pub id: Id<RelationshipMarker>,
    /// A custom nickname set for the related user.
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<Timestamp<Iso8601>>,
    #[serde(rename = "type")]
    pub r#type: RelationshipType,
    pub user: UserPartial,
}
