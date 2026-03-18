use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::RelationshipMarker};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct RelationshipRemove {
    pub id: Id<RelationshipMarker>,
}
