use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::ChannelMarker},
    user::UserPartial,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelRecipientAdd {
    pub channel_id: Id<ChannelMarker>,
    pub user: UserPartial,
}
