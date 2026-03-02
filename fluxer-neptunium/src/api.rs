use fluxer_gateway::model::snowflake::Snowflake;

use crate::api::messages::MessageReference;

pub mod messages;

pub(crate) enum ApiClientMessage {
    SendMessage {
        channel_id: Snowflake,
        content: String,
        reference: Option<MessageReference>,
    },
}

pub struct ApiClient<'a> {
    pub base_path: &'a str,
    pub user_agent: String,
    pub token: zeroize::Zeroizing<String>,
    pub reqwest_client: reqwest::Client,
}

impl ApiClient<'_> {
    pub(crate) async fn on_message(&mut self, message: ApiClientMessage) {
        match message {
            ApiClientMessage::SendMessage {
                channel_id,
                content,
                reference,
            } => self.send_message(channel_id, content, reference).await,
        }
    }
}
