use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::ChannelMarker};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct CallDelete {
    pub channel_id: Id<ChannelMarker>,
}
