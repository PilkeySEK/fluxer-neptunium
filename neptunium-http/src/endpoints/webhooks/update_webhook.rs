use bon::Builder;
use neptunium_model::{
    guild::webhook::Webhook,
    id::{
        Id,
        marker::{ChannelMarker, WebhookMarker},
    },
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Builder, Clone, Debug)]
pub struct UpdateWebhookBody {
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // TODO: Check whether webhook avatars can be removed, and if yes, how?
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateWebhook {
    pub webhook_id: Id<WebhookMarker>,
    pub body: UpdateWebhookBody,
}

impl Endpoint for UpdateWebhook {
    type Response = Webhook;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/webhooks/{}", self.webhook_id))
            .build()
    }
}
