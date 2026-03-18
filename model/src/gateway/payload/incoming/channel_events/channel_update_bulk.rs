use serde::{Deserialize, Serialize};

use crate::channel::Channel;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelUpdateBulk {
    pub channels: Vec<Channel>,
}
