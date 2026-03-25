//! Demonstration of how to use user-bots (logging in as a user). You need to have the "user_api" feature enabled on fluxer-neptunium.

use std::{env, sync::Arc};

use fluxer_neptunium::{
    http::client::TokenType,
    model::gateway::payload::incoming::{
        passive_updates::PassiveUpdates, session_events::ready::Ready,
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_ready(&self, _ctx: Context, data: Arc<Ready>) -> Result<(), EventError> {
        println!(
            "Logged in as {}#{}",
            data.user.username, data.user.discriminator
        );
        Ok(())
    }

    async fn on_passive_updates(
        &self,
        _ctx: Context,
        data: Arc<PassiveUpdates>,
    ) -> Result<(), EventError> {
        println!("Passive updates received: {:?}", data);

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::new_with_config(
        ShardConfig::builder()
            .token(env::var("FLUXER_USER_TOKEN").unwrap())
            .build(),
        ClientConfig::builder().token_type(TokenType::User).build(),
    );

    client.register_event_handler(Handler);

    client.start().await.unwrap();
}
