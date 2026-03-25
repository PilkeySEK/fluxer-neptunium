use std::{env, sync::Arc};

use fluxer_neptunium::{
    model::gateway::{
        intents::GatewayEventFlags,
        payload::incoming::message_events::message_create::MessageCreate,
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_message_create(
        &self,
        ctx: Context,
        message: Arc<MessageCreate>,
    ) -> Result<(), EventError> {
        if !message.author.bot && message.content == "n?ping" {
            message.reply(&ctx, "Pong!").await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("FLUXER_TOKEN").unwrap();
    let mut client = Client::new(
        ShardConfig::builder()
            .token(token)
            .ignored_events(GatewayEventFlags::GUILDS | GatewayEventFlags::GUILD_MESSAGES)
            .build(),
    );

    client.register_event_handler(Handler);

    client.start().await.unwrap();
}
