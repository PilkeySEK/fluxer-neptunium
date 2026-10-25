use bon::Builder;
use neptunium_model::id::{Id, marker::WebhookMarker};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct DeleteWebhook {
    pub webhook_id: Id<WebhookMarker>,
}

impl Endpoint for DeleteWebhook {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!("/webhooks/{}", self.webhook_id))
            .build()
    }
}
