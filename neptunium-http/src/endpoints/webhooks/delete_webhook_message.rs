use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{MessageMarker, WebhookMarker},
};
use reqwest::Method;
use zeroize::Zeroizing;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DeleteWebhookMessage {
    pub webhook_id: Id<WebhookMarker>,
    #[builder(into)]
    pub token: Zeroizing<String>,
    pub message_id: Id<MessageMarker>,
}

impl Endpoint for DeleteWebhookMessage {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .use_authorization_token(false)
            .method(Method::DELETE)
            .path(format!(
                "/webhooks/{}/{}/messages/{}",
                self.webhook_id, *self.token, self.message_id
            ))
            .build()
    }
}
