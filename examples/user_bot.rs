//! Demonstration of how to use user-bots (logging in as a user). You need to have the "user_api" feature enabled on fluxer-neptunium.

use std::{env, sync::Arc};

use fluxer_neptunium::{
    cached_payload::CachedReady, http::client::TokenType,
    model::gateway::payload::incoming::PassiveUpdates, prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_ready(&self, _ctx: Context, data: Arc<CachedReady>) -> Result<(), EventError> {
        let user = data.user.read().await;
        println!("Logged in as {}#{}", user.username, user.discriminator);
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
        env::var("FLUXER_USER_TOKEN").unwrap(),
        ClientConfig::builder().token_type(TokenType::User).build(),
    );

    client.register_event_handler(Handler);

    client.start().await.unwrap();
}
