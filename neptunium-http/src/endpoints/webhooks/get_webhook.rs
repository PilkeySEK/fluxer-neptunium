use bon::Builder;
use neptunium_model::{
    guild::webhook::Webhook,
    id::{Id, marker::WebhookMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetWebhook {
    pub webhook_id: Id<WebhookMarker>,
}

impl Endpoint for GetWebhook {
    type Response = Webhook;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/webhooks/{}", self.webhook_id))
            .build()
    }
}
