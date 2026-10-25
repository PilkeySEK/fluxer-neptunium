use std::sync::Arc;

use neptunium_model::id::{Id, marker::ChannelMarker};
use neptunium_http::{builders::create_message_builder::CreateMessageBuilder, client::HttpClient};

/// Operations on messages in a channel.
pub struct MessagesClient {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) channel_id: Id<ChannelMarker>,
}

impl MessagesClient {
    /// Returns a builder for creating a message in the channel.
    pub fn create(&self) -> CreateMessageBuilder {
        CreateMessageBuilder::new(Arc::clone(&self.http_client), self.channel_id)
    }
}
