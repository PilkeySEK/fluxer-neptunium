use async_trait::async_trait;
use fluxer_model::channel::message::Message;
use reqwest::{Method, StatusCode};

use crate::{client::error::Error, events::context::Context};

#[async_trait]
pub trait MessageExt {
    async fn reply(&self, ctx: &Context, content: &str) -> Result<(), crate::client::error::Error>;
}

#[async_trait]
impl MessageExt for Message {
    async fn reply(&self, ctx: &Context, content: &str) -> Result<(), crate::client::error::Error> {
        let url = format!(
            "{}/channels/{}/messages",
            ctx.api_info.base_path, self.channel_id
        );
        let response = ctx
            .api_info
            .client
            .request(Method::POST, &url)
            .header("Authorization", format!("Bot {}", *ctx.api_info.token))
            .header("User-Agent", &ctx.api_info.user_agent)
            .body(format!(r#"{{"content":"{content}"}}"#)) // TODO
            .send()
            .await?;
        if response.status() != StatusCode::OK {
            return Err(Error::new(
                crate::client::error::ClientErrorKind::HttpStatusNot200(response),
            ));
        }
        Ok(())
    }
}
