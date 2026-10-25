use bon::Builder;
use neptunium_model::id::{Id, marker::WebhookMarker};
use reqwest::Method;
use zeroize::Zeroizing;

use crate::{
    endpoints::{
        Endpoint,
        webhooks::{
            get_webhook_with_token::GetWebhookWithTokenResponse, update_webhook::UpdateWebhookBody,
        },
    },
    request::Request,
};

#[derive(Builder, Clone, Debug)]
pub struct UpdateWebhookWithToken {
    pub webhook_id: Id<WebhookMarker>,
    pub token: Zeroizing<String>,
    pub body: UpdateWebhookBody,
}

impl Endpoint for UpdateWebhookWithToken {
    type Response = GetWebhookWithTokenResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .use_authorization_token(false)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/webhooks/{}/{}", self.webhook_id, *self.token))
            .build()
    }
}
