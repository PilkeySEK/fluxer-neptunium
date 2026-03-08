use async_trait::async_trait;
use fluxer_model::channel::message::{Message, MessageReferenceType};
use reqwest::StatusCode;

use crate::{client::error::Error, events::context::Context};

use neptunium_http::channel::messages::{
    message_create::CreateMessageBody, message_reference::MessageReference,
};

#[async_trait]
pub trait MessageExt {
    async fn reply(
        &self,
        ctx: &Context,
        data: CreateMessageBody,
    ) -> Result<(), crate::client::error::Error>;
}

#[async_trait]
impl MessageExt for Message {
    async fn reply(
        &self,
        ctx: &Context,
        mut data: CreateMessageBody,
    ) -> Result<(), crate::client::error::Error> {
        // let url = format!(
        //     "{}/channels/{}/messages",
        //     ctx.api_info.base_path, self.channel_id
        // );
        // let response = ctx
        //     .api_info
        //     .client
        //     .request(Method::POST, &url)
        //     .header("Authorization", format!("Bot {}", *ctx.api_info.token))
        //     .header("User-Agent", &ctx.api_info.user_agent)
        //     .body(format!(r#"{{"content":"{content}"}}"#)) // TODO
        //     .send()
        //     .await?;
        data.message_reference = Some(MessageReference {
            message_id: self.id,
            channel_id: Some(self.channel_id),
            guild_id: None,
            r#type: MessageReferenceType::Reply,
        });
        let response = ctx
            .api_client
            .messages(self.channel_id)
            .create(&data)
            .await?;
        if response.status() != StatusCode::OK {
            return Err(Error::new(
                crate::client::error::ClientErrorKind::HttpStatusNot200(response),
            ));
        }
        Ok(())
    }
}
