use std::{collections::HashMap, convert::Infallible, future, sync::Arc};

use neptunium_cache_inmemory::Cache;
use neptunium_gateway::session::{Shard, config::SessionConfig};

use neptunium_http::client::HttpClient;
use tokio::sync::mpsc::unbounded_channel;
use tokio_util::sync::CancellationToken;
use tracing::instrument;

use crate::{
    client::session::Session,
    events::{EventHandler, context::Context},
};

mod config;
pub use config::*;
pub(crate) mod error;
pub use error::*;

struct ClientInternalConfig {
    #[cfg(feature = "user_api")]
    subscribe_to_everything: bool,
}

pub struct Client {
    // shard_config: ShardConfig,
    context: Context,
    event_handlers: Vec<Box<dyn EventHandler + Sync>>,
    config: ClientInternalConfig,
    shard: Shard,
}

impl Client {
    /// Create a new client provided a shard config.
    ///
    /// # Examples
    /// ```
    /// # use crate::client::Client;
    /// # use neptunium_gateway::shard::config::ShardConfig;
    /// # fn main() {
    /// let client = Client::new("my bot token");
    /// let client = Client::new(ShardConfig::builder().token("my bot token").build());
    /// # }
    /// ```
    #[must_use]
    pub fn new(shard_config: impl Into<SessionConfig>) -> Self {
        Self::new_with_config(shard_config, ClientConfig::default())
    }

    #[must_use]
    pub fn new_with_config(
        shard_config: impl Into<SessionConfig>,
        client_config: ClientConfig,
    ) -> Self {
        let shard_config = shard_config.into();

        let (tx, rx) = unbounded_channel();

        Self {
            shard: Shard::new(shard_config),
            context: Context {
                http_client: Arc::new({
                    let mut api_client = HttpClient::builder()
                        .token(shard_config.token.clone())
                        .token_type(client_config.token_type)
                        .maybe_bot_user_agent(client_config.bot_user_agent_information)
                        .build();
                    if let Some(api_base_url) = client_config.api_base_url {
                        api_client.api_base_url = api_base_url;
                    }
                    api_client
                }),
                tx,
                cache: Arc::new(Cache::new(client_config.cache_config)),
                default_allowed_mentions: Arc::new(client_config.default_allowed_mentions),
            },
            event_handlers: Vec::new(),
            config: ClientInternalConfig {
                #[cfg(feature = "user_api")]
                subscribe_to_everything: client_config.subscribe_to_everything,
            },
        }
    }

    #[must_use]
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Register a new event handler.
    ///
    /// # Examples
    /// ```rs
    /// use std::sync::Arc;
    /// use fluxer_neptunium::prelude::*;
    ///
    /// struct Handler;
    ///
    /// #[async_trait]
    /// impl EventHandler for Handler {
    ///     async fn on_ready(&self, _ctx: Context, event: Arc<CachedReady>)
    ///         -> Result<(), EventError>
    ///     {
    ///         tracing::info!(
    ///             "Logged in as {}#{}",
    ///             event.user.username,
    ///             event.user.discriminator
    ///         );
    ///         Ok(())
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("my_token");
    ///     client.register_event_handler(Handler);
    ///     // ...
    /// }
    /// ```
    pub fn register_event_handler(&mut self, handler: impl EventHandler + Sync + 'static) {
        self.event_handlers.push(Box::new(handler));
    }

    pub async fn start(&mut self) -> Result<Infallible, Error> {
        match self
            .start_cancellable(future::pending::<Infallible>())
            .await
        {
            Err(e) => Err(e),
            // This branch can never happen because it would require constructing `Infallible`
            Ok((_, infallible)) => match infallible {},
        }
    }

    /// Start the client and stop the client when the provided `cancel` future is fulfilled,
    /// returning `ResumeInfo` (if it is available) and the return value of the future.
    pub async fn start_cancellable<T: Send + 'static>(
        &mut self,
        cancel: impl Future<Output = T> + Send + 'static,
    ) -> Result<(Option<ResumeInfo>, T), Error> {
        let all_cancellation_token = CancellationToken::new();
        let cancel_future_task = tokio::spawn(cancel);
        let resume_info = None;
        loop {
            tokio::select! {
                maybe_cancel_future_result = cancel_future_task => {
                    all_cancellation_token.cancel();
                    break match maybe_cancel_future_result {
                        Ok(cancel_future_result) => Ok((resume_info, cancel_future_result)),
                        Err(e) => Err(Error::new(ClientErrorKind::CancelFutureJoinError(e))),
                    };
                },
                session_result = self.session() => {
                    todo!()
                }
            }
        }
    }

    #[instrument(skip(self))]
    async fn session(&mut self) {
        let session_cancellation_token = CancellationToken::new();
        tracing::debug!("Starting new client session.");
        #[cfg(feature = "user_api")]
        if self.config.subscribe_to_everything {
            let context = self.context.clone();
            tokio::spawn(async move {
                match session_cancellation_token
                    .run_until_cancelled(Self::subscribe_to_everything(context))
                    .await
                {
                    Some(Err(e)) => {
                        tracing::error!("Failed to subscribe to all events: {e}");
                    }
                    None => {
                        tracing::warn!(
                            "Cancelled subscribing to all events because the session stopped."
                        );
                    }
                    Some(Ok(())) => {}
                }
            });
        }

        let session = Session {
            shard: &mut self.shard,
        };
    }

    #[cfg(feature = "user_api")]
    #[tracing::instrument(skip(ctx))]
    async fn subscribe_to_everything(ctx: Context) -> Result<(), Error> {
        use neptunium_model::gateway::payload::outgoing::GuildSubscriptionRequest;

        tracing::debug!("Subscribing to all guild events.");
        let guilds = ctx.list_own_guilds().await?;

        let subscriptions = guilds
            .into_iter()
            .map(|guild| {
                (
                    guild.id,
                    GuildSubscriptionRequest {
                        active: Some(true),
                        member_list_channels: None,
                        typing: None,
                        members: None,
                        sync: Some(true),
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        ctx.update_guild_event_subscriptions(subscriptions).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_client_is_send_sync() {
        static_assertions::assert_impl_all!(Client: Send, Sync);
    }
}
