use fluxer_gateway::model::snowflake::Snowflake;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::ApiClient;

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum MessageReferenceType {
    Reply = 0,
    Forward = 1,
}

#[derive(Serialize, Deserialize)]
pub struct MessageReference {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub r#type: MessageReferenceType,
}

impl ApiClient<'_> {
    pub async fn send_message(
        &self,
        channel_id: Snowflake,
        content: String,
        reference: Option<MessageReference>,
    ) {
        let json; // = json!({
        //    "content": content,
        //});
        if let Some(reference) = reference {
            json = json!({
                "content": content,
                "message_reference": reference,
            });
        } else {
            json = json!({
                "content": content,
            });
        }
        let request = self
            .reqwest_client
            .request(
                Method::POST,
                format!(
                    "{}/channels/{}/messages",
                    self.base_path,
                    String::from(channel_id)
                ),
            )
            .header("Authorization", format!("Bot {}", self.token.as_str()))
            .header("User-Agent", &self.user_agent)
            .body(serde_json::to_string(&json).unwrap())
            .build()
            .unwrap();
        self.reqwest_client.execute(request).await.unwrap();
    }
}
