use bon::Builder;
use neptunium_model::{
    guild::webhook::Webhook,
    id::{Id, marker::ChannelMarker},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct CreateWebhook {
    pub channel_id: Id<ChannelMarker>,
    /// The name of the webhook.
    pub name: String,
    /// The avatar image as a base64-encoded data URI.
    pub avatar: Option<String>,
}

impl Endpoint for CreateWebhook {
    type Response = Webhook;

    fn into_request(self) -> crate::request::Request {
        #[derive(Serialize)]
        struct CreateWebhookBody {
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            avatar: Option<String>,
        }

        let body = CreateWebhookBody {
            name: self.name,
            avatar: self.avatar,
        };

        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&body).unwrap())
            .path(format!("/channels/{}/webhooks", self.channel_id))
            .build()
    }
}
