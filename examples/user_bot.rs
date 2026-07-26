//! Demonstration of how to use user-bots (logging in as a user). You need to have the "user_api" feature enabled on fluxer-neptunium.

use std::{env, sync::Arc};

use fluxer_neptunium::{
    cached_payload::{CachedMessageCreate, CachedReady},
    http::client::TokenType,
    model::gateway::payload::incoming::PassiveUpdates,
    prelude::*,
};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_ready(&self, _ctx: Context, event: Arc<CachedReady>) -> Result<(), EventError> {
        tracing::info!(
            "Logged in as {}#{}",
            event.user.username,
            event.user.discriminator
        );
        Ok(())
    }

    async fn on_message_create(
        &self,
        ctx: Context,
        event: Arc<CachedMessageCreate>,
    ) -> Result<(), EventError> {
        let author_string = format!(
            "{}#{}",
            event.message.author.username, event.message.author.discriminator
        );
        tracing::info!(
            author = %author_string,
            "{}",
            event.message.content
        );
        let current_user_id = ctx.get_own_profile().await?.id;
        if event.message.author.id == current_user_id && event.message.content == "u?ping" {
            event.message.reply(&ctx, "Pong!").await?;
        }
        Ok(())
    }

    // PASSIVE_UPDATES is a user-only gateway dispatch event
    async fn on_passive_updates(
        &self,
        _ctx: Context,
        _event: Arc<PassiveUpdates>,
    ) -> Result<(), EventError> {
        // println!("Passive updates received: {:?}", event);

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let mut client = Client::new_with_config(
        env::var("FLUXER_USER_TOKEN").unwrap(),
        ClientConfig::builder()
            .token_type(TokenType::User)
            .subscribe_to_everything(true)
            .build(),
    );
    client.register_event_handler(Handler);
    client.start().await.unwrap();
}
