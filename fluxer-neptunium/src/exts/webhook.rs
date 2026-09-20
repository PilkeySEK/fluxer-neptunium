use async_trait::async_trait;
use neptunium_http::endpoints::webhooks::{
    DeleteWebhook, DeleteWebhookMessage, DeleteWebhookWithToken, ExecuteWebhook, GetWebhook,
    GetWebhookWithToken, GetWebhookWithTokenResponse, UpdateWebhook, UpdateWebhookBody,
    WebhookMessage,
};
use neptunium_model::{
    channel::message::Message,
    guild::webhook::Webhook,
    id::{
        Id,
        marker::{MessageMarker, WebhookMarker},
    },
};
use zeroize::Zeroizing;

use crate::{client::error::ClientError, events::context::Context};

#[async_trait]
pub trait WebhookExt {
    /// Fetch webhook information from the API.
    async fn fetch(&self, ctx: &Context) -> Result<Webhook, ClientError>;
    async fn delete(&self, ctx: &Context) -> Result<(), ClientError>;
    async fn update(
        &self,
        ctx: &Context,
        updates: UpdateWebhookBody,
    ) -> Result<Webhook, ClientError>;
    /// Fetch the webhook using the webhook token (not the bot token).
    async fn fetch_with_token(
        &self,
        ctx: &Context,
        token: String,
    ) -> Result<GetWebhookWithTokenResponse, ClientError>;
    async fn execute(
        &self,
        ctx: &Context,
        token: String,
        message: WebhookMessage,
    ) -> Result<(), ClientError>;
    /// Execute the webhook and wait for a message response.
    async fn execute_and_wait(
        &self,
        ctx: &Context,
        token: String,
        message: WebhookMessage,
    ) -> Result<Message, ClientError>;
    async fn delete_with_token(
        &self,
        ctx: &Context,
        token: impl Into<Zeroizing<String>> + Send,
    ) -> Result<(), ClientError>;
    async fn delete_message(
        &self,
        ctx: &Context,
        token: impl Into<Zeroizing<String>> + Send,
        message_id: Id<MessageMarker>,
    ) -> Result<(), ClientError>;
}

#[async_trait]
impl WebhookExt for Id<WebhookMarker> {
    async fn fetch(&self, ctx: &Context) -> Result<Webhook, ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(GetWebhook { webhook_id: *self })
            .await?)
    }

    async fn delete(&self, ctx: &Context) -> Result<(), ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(DeleteWebhook { webhook_id: *self })
            .await?)
    }

    async fn update(
        &self,
        ctx: &Context,
        updates: UpdateWebhookBody,
    ) -> Result<Webhook, ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(UpdateWebhook {
                webhook_id: *self,
                body: updates,
            })
            .await?)
    }

    async fn fetch_with_token(
        &self,
        ctx: &Context,
        token: String,
    ) -> Result<GetWebhookWithTokenResponse, ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(GetWebhookWithToken {
                webhook_id: *self,
                token: Zeroizing::new(token),
            })
            .await?)
    }

    async fn execute(
        &self,
        ctx: &Context,
        token: String,
        message: WebhookMessage,
    ) -> Result<(), ClientError> {
        let None = ctx
            .get_http_client()
            .execute(ExecuteWebhook {
                webhook_id: *self,
                token: Zeroizing::new(token),
                message,
                wait: false,
            })
            .await?
        else {
            return Err(ClientError::HttpInvalidResponse(
                "Expected no content".to_owned(),
            ));
        };
        Ok(())
    }

    async fn execute_and_wait(
        &self,
        ctx: &Context,
        token: String,
        message: WebhookMessage,
    ) -> Result<Message, ClientError> {
        let Some(response) = ctx
            .get_http_client()
            .execute(ExecuteWebhook {
                webhook_id: *self,
                token: Zeroizing::new(token),
                message,
                wait: true,
            })
            .await?
        else {
            return Err(ClientError::HttpInvalidResponse(
                "Expected a message".to_owned(),
            ));
        };
        Ok(response)
    }

    async fn delete_with_token(
        &self,
        ctx: &Context,
        token: impl Into<Zeroizing<String>> + Send,
    ) -> Result<(), ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(DeleteWebhookWithToken {
                webhook_id: *self,
                token: token.into(),
            })
            .await?)
    }

    async fn delete_message(
        &self,
        ctx: &Context,
        token: impl Into<Zeroizing<String>> + Send,
        message_id: Id<MessageMarker>,
    ) -> Result<(), ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(DeleteWebhookMessage {
                webhook_id: *self,
                token: token.into(),
                message_id,
            })
            .await?)
    }
}
