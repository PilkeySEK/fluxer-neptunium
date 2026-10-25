use bon::Builder;
use neptunium_model::{
    channel::message::Message,
    id::{
        Id,
        marker::{MessageMarker, WebhookMarker},
    },
};
use reqwest::Method;
use zeroize::Zeroizing;

use crate::{
    endpoints::{Endpoint, channel::EditMessageBody},
    request::Request,
};

#[derive(Builder, Clone, Debug)]
pub struct EditWebhookMessage {
    pub webhook_id: Id<WebhookMarker>,
    #[builder(into)]
    pub token: Zeroizing<String>,
    pub message_id: Id<MessageMarker>,
    pub body: EditMessageBody,
}

impl Endpoint for EditWebhookMessage {
    type Response = Message;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .use_authorization_token(false)
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!(
                "/webhooks/{}/{}/messages/{}",
                self.webhook_id, *self.token, self.message_id
            ))
            .build()
    }
}
