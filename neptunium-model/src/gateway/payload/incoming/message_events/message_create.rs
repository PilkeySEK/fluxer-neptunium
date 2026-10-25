use serde::{Deserialize, Serialize};

use crate::channel::{ChannelType, message::Message};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageCreate {
    #[serde(flatten)]
    pub message: Message,
    pub channel_type: ChannelType,
}

impl std::ops::Deref for MessageCreate {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl std::ops::DerefMut for MessageCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.message
    }
}
