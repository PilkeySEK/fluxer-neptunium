use std::{env, sync::Arc, time::SystemTime};

use fluxer_neptunium::{
    cached_payload::CachedMessageCreate,
    model::{
        gateway::{payload::outgoing::InitialPresence, presence::CustomStatus},
        time::OffsetDateTime,
    },
    prelude::*,
};
use tracing_subscriber::filter::LevelFilter;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_message_create(
        &self,
        ctx: Context,
        event: Arc<CachedMessageCreate>,
    ) -> Result<(), EventError> {
        let now = SystemTime::now();
        let message_latency = now
            .duration_since(SystemTime::from(OffsetDateTime::from(
                event.message.timestamp,
            )))
            .map_or("<error>".to_owned(), |duration| {
                duration.as_millis().to_string()
            });
        if !event.message.author.bot && event.message.content == "n?ping" {
            event
                .message
                .reply(&ctx, format!("Pong! API latency: {message_latency} ms"))
                .await?;
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
    let mut client = Client::new(
        SessionConfig::builder()
            .token(token)
            .initial_presence(
                InitialPresence::builder()
                    .custom_status(CustomStatus::builder().text("Fluxin' it").build())
                    .build(),
            )
            .send_initial_presence_on_every_reconnect(true)
            .build(),
    );

    client.register_event_handler(Handler);

    client.start().await.unwrap();
}
