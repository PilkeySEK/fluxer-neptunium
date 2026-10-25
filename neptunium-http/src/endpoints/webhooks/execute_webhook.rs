use std::collections::HashMap;

use bon::Builder;
use neptunium_model::{
    channel::message::Message,
    id::{Id, marker::WebhookMarker},
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

#[derive(Builder, Clone, Debug)]
pub struct ExecuteWebhook {
    pub webhook_id: Id<WebhookMarker>,
    #[builder(into)]
    pub token: Zeroizing<String>,
    pub message: WebhookMessage,
    #[builder(default = false)]
    pub wait: bool,
}

impl Endpoint for ExecuteWebhook {
    type Response = Option<Message>;

    fn into_request(self) -> crate::request::Request {
        let params = if self.wait {
            let mut hashmap = HashMap::new();
            hashmap.insert("wait".to_owned(), "true".to_owned());
            Some(hashmap)
        } else {
            None
        };

        Request::builder()
            .method(Method::POST)
            .use_authorization_token(false)
            .body(serde_json::to_string(&self.message).unwrap())
            .path(format!("/webhooks/{}/{}", self.webhook_id, *self.token))
            .maybe_params(params)
            .build()
    }
}
