use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::ChannelMarker},
    user::PartialUser,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelRecipientRemove {
    pub channel_id: Id<ChannelMarker>,
    pub user: PartialUser,
}
