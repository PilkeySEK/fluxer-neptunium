use bon::Builder;
use neptunium_model::id::{Id, marker::WebhookMarker};
use reqwest::Method;
use zeroize::Zeroizing;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DeleteWebhookWithToken {
    pub webhook_id: Id<WebhookMarker>,
    #[builder(into)]
    pub token: Zeroizing<String>,
}

impl Endpoint for DeleteWebhookWithToken {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .use_authorization_token(false)
            .path(format!("/webhooks/{}/{}", self.webhook_id, *self.token))
            .build()
    }
}
