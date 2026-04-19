use std::{env, sync::Arc};

use fluxer_neptunium::{
    cached_payload::{CachedMessageCreate, CachedReady},
    events::context::FileUploadParams,
    http::endpoints::channel::{AttachmentBase, CreateMessageBody},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_ready(&self, _ctx: Context, event: Arc<CachedReady>) -> Result<(), EventError> {
        let user = event.user.load();
        println!(
            "Ready! Logged in as {}#{}",
            user.username, user.discriminator
        );
        Ok(())
    }

    async fn on_message_create(
        &self,
        ctx: Context,
        event: Arc<CachedMessageCreate>,
    ) -> Result<(), EventError> {
        let file = b"abcdefg".to_vec();
        let message = event.message.load();
        if message.content != "n?do-upload" {
            return Ok(());
        }
        let attachments = ctx
            .upload_files(
                message.channel_id,
                vec![
                    FileUploadParams::builder()
                        .file_data(file)
                        .filename("thing.txt")
                        .id(0)
                        .attachment(AttachmentBase::builder().build())
                        .build(),
                ],
            )
            .await?;

        message
            .reply(
                &ctx,
                CreateMessageBody::builder()
                    .content("Here's a yummy file :yum:")
                    .attachments(attachments)
                    .build(),
            )
            .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::new(env::var("FLUXER_TOKEN").unwrap());
    client.register_event_handler(Handler);
    if let Err(e) = client.start().await {
        println!("Client Error: {e}");
    }
}
