use bon::Builder;
use neptunium_model::{
    channel::message::Message,
    id::{
        Id,
        marker::{ChannelMarker, WebhookMarker},
    },
};
use reqwest::Method;
use serde::Serialize;
use zeroize::Zeroizing;

use crate::{
    endpoints::{Endpoint, channel::CreateMessageBody},
    request::Request,
};

// Source: https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/api/src/webhook/WebhookService.tsx#L64
// TODO: Check whether this struct is correct.
// https://docs.fluxer.com/http-api/webhooks/#webhook-message-body ?
#[derive(Builder, Serialize, Clone, Debug)]
pub struct WebhookMessage {
    #[serde(flatten)]
    pub base: CreateMessageBody,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

#[derive(Builder, serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub struct ExecuteWebhookParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    /// The ID of a thread in the webhook's channel to post in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<Id<ChannelMarker>>,
}

#[derive(Builder, Clone, Debug)]
pub struct ExecuteWebhook {
    pub webhook_id: Id<WebhookMarker>,
    #[builder(into)]
    pub token: Zeroizing<String>,
    pub message: WebhookMessage,
    pub params: ExecuteWebhookParams,
}

impl Endpoint for ExecuteWebhook {
    // TODO: Probably add a constant WAIT parameter to ExecuteWebhook to have either () or Message here.
    type Response = Option<Message>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .use_authorization_token(false)
            .body(serde_json::to_string(&self.message).unwrap())
            .path(format!("/webhooks/{}/{}", self.webhook_id, *self.token))
            .params(serde_qs::to_string(&self.params).unwrap())
            .build()
    }
}
