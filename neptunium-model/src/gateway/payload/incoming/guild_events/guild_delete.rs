use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::GuildMarker};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildDelete {
    // This was specified as `string` in the documentation, but a guild ID should be a `Snowflake`
    /// The ID of the guild which was deleted.
    pub id: Id<GuildMarker>,
    /// True if the guild is unavailable due to an outage (not a leave/kick).
    #[serde(default)]
    pub unavailable: bool,
}
