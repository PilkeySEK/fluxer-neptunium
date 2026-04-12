use std::{env, sync::Arc, time::Duration};

use fluxer_neptunium::{cached_payload::CachedMessageCreate, prelude::*};
use tracing_subscriber::filter::LevelFilter;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_message_create(
        &self,
        ctx: Context,
        event: Arc<CachedMessageCreate>,
    ) -> Result<(), EventError> {
        let message = event.message.load();
        let author = message.author.load();
        if !author.bot && message.content == "n?ping" {
            message.reply(&ctx, "Pong!").await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    let token = env::var("FLUXER_TOKEN").unwrap();
    let mut client = Client::new_with_config(
        token,
        ClientConfig::builder()
            // Convenience when testing reconnecting (e.g. by turning off internet and back on),
            // will make the reconnection process faster
            .auto_reconnect_wait_time(Duration::from_secs(5))
            .build(),
    );

    client.register_event_handler(Handler);

    client.start().await.unwrap();
}
