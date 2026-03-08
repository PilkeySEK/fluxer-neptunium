use fluxer_model::id::{Id, marker::ChannelMarker};
use reqwest::{Method, Response};

use crate::{channel::messages::message_create::CreateMessageBody, client::HttpClient};

pub struct MessagesClient<'a> {
    pub(super) client: &'a HttpClient,
    pub(super) channel_id: Id<ChannelMarker>,
}

impl MessagesClient<'_> {
    /// Create a message.
    /// # Errors
    /// Returns an error if the request fails.
    #[expect(clippy::missing_panics_doc)]
    pub async fn create(&self, data: &CreateMessageBody) -> Result<Response, reqwest::Error> {
        let body = serde_json::to_string(data).unwrap();

        self.client
            .reqwest_client
            .request(
                Method::POST,
                format!(
                    "{}/channels/{}/messages",
                    self.client.api_base_url, self.channel_id
                ),
            )
            .header("Authorization", format!("Bot {}", *self.client.token))
            .header("User-Agent", &self.client.user_agent)
            .body(body)
            .send()
            .await
    }
}
