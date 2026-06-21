use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, GuildMarker, WebhookMarker},
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct GetWebhookWithToken {
    pub webhook_id: Id<WebhookMarker>,
    #[builder(into)]
    pub token: Zeroizing<String>,
}

/// A `Webhook` struct without the creator user data.
#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct GetWebhookWithTokenResponse {
    pub id: Id<WebhookMarker>,
    pub guild_id: Id<GuildMarker>,
    pub channel_id: Id<ChannelMarker>,
    /// The display name.
    pub name: String,
    pub token: Zeroizing<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl Endpoint for GetWebhookWithToken {
    type Response = GetWebhookWithTokenResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .use_authorization_token(false)
            .path(format!("/webhooks/{}/{}", self.webhook_id, *self.token))
            .build()
    }
}
