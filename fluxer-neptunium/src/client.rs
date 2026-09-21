use std::{
    collections::HashMap,
    convert::Infallible,
    future,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use neptunium_cache_inmemory::{Cache, gateway::cached_payload::CachedGuildMembersChunk};
use neptunium_gateway::session::{ResumeInfo, Session, SessionHandle, config::SessionConfig};

use neptunium_http::client::HttpClient;
use neptunium_model::gateway::payload::{
    incoming::GuildCountsUpdate,
    outgoing::{
        LazyRequest, OutgoingGatewayMessage, PresenceUpdateOutgoing, RequestGuildCounts,
        RequestGuildMembers,
    },
};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    oneshot,
};
use tokio_util::sync::CancellationToken;

use crate::events::{EventHandler, context::Context};

mod config;
pub use config::*;
pub(crate) mod error;
pub use error::*;
mod dispatch_event_impl;

struct ClientInternalConfig {
    #[cfg(feature = "user_api")]
    subscribe_to_everything: bool,
}

pub(crate) enum ClientMessage {
    UpdatePresence(PresenceUpdateOutgoing, oneshot::Sender<bool>),
    RequestGuildMembers(
        RequestGuildMembers,
        oneshot::Sender<bool>,
        Option<UnboundedSender<CachedGuildMembersChunk>>,
    ),
    SendLazyRequest(LazyRequest, UnboundedSender<()>),
    RequestGuildCounts(
        RequestGuildCounts,
        oneshot::Sender<bool>,
        Option<oneshot::Sender<GuildCountsUpdate>>,
    ),
}

enum SessionTaskMessage {
    Send(OutgoingGatewayMessage),
    SendWithResultOneshot(OutgoingGatewayMessage, oneshot::Sender<bool>),
    SendWithResultUnbounded(OutgoingGatewayMessage, UnboundedSender<bool>),
}

pub struct Client {
    // shard_config: ShardConfig,
    context: Context,
    event_handlers: Vec<Arc<dyn EventHandler + Sync>>,
    config: ClientInternalConfig,
    session_config: SessionConfig,
    guild_members_chunk_listeners: HashMap<String, UnboundedSender<CachedGuildMembersChunk>>,
    guild_counts_update_listeners: HashMap<String, oneshot::Sender<GuildCountsUpdate>>,
    context_rx: UnboundedReceiver<ClientMessage>,
}

impl Deref for Client {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
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
    pub fn new(session_config: impl Into<SessionConfig>) -> Self {
        Self::new_with_config(session_config, ClientConfig::default())
    }

    #[must_use]
    pub fn new_with_config(
        session_config: impl Into<SessionConfig>,
        client_config: ClientConfig,
    ) -> Self {
        let session_config = session_config.into();

        let (context_tx, context_rx) = unbounded_channel();

        Self {
            context: Context {
                http_client: Arc::new({
                    let mut api_client = HttpClient::builder()
                        .token(session_config.token.clone())
                        .token_type(client_config.token_type)
                        .maybe_bot_user_agent(client_config.bot_user_agent_information)
                        .build();
                    if let Some(api_base_url) = client_config.api_base_url {
                        api_client.api_base_url = api_base_url;
                    }
                    api_client
                }),
                tx: context_tx,
                cache: Arc::new(Cache::new(client_config.cache_config)),
                default_allowed_mentions: Arc::new(client_config.default_allowed_mentions),
            },
            session_config,
            event_handlers: Vec::new(),
            config: ClientInternalConfig {
                #[cfg(feature = "user_api")]
                subscribe_to_everything: client_config.subscribe_to_everything,
            },
            guild_counts_update_listeners: HashMap::new(),
            guild_members_chunk_listeners: HashMap::new(),
            context_rx,
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
        self.register_arc_event_handler(Arc::new(handler));
    }

    /// Same as [`register_event_handler`], except you can pass the event handler as an `Arc`.
    ///
    /// [`register_event_handler`]: Self::register_event_handler
    pub fn register_arc_event_handler(&mut self, handler: Arc<dyn EventHandler + Sync + 'static>) {
        self.event_handlers.push(handler);
    }

    pub async fn start(&mut self) -> Result<Infallible, ClientError> {
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
    pub async fn start_cancellable<T: Send + Sync + 'static>(
        &mut self,
        cancel: impl Future<Output = T> + Send + 'static,
    ) -> Result<(Option<ResumeInfo>, T), ClientError> {
        let cancellation_token = CancellationToken::new();
        let cancellation_token_drop_guard = cancellation_token.clone().drop_guard();
        #[cfg(feature = "user_api")]
        if self.config.subscribe_to_everything {
            let context = self.context.clone();
            tokio::spawn(async move {
                match cancellation_token
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

        let (event_tx, mut event_rx) = unbounded_channel();
        let session_config = self.session_config.clone();
        let mut session = Session::connect(session_config)
            .await
            .map_err(ClientError::GatewayConnectError)?;
        let handle = session.handle();
        let mut session_task = tokio::spawn(async move {
            session
                .run_cancellable(
                    |event| {
                        if let Err(err) = event_tx.send(event) {
                            tracing::warn!(%err, "Failed to send event over channel");
                        }
                    },
                    cancel,
                )
                .await
                .map_err(ClientError::GatewaySessionError)
        });
        let session_task_result = loop {
            tokio::select! {
                result = &mut session_task => {
                    break result;
                }
                maybe_event = event_rx.recv() => {
                    let Some(event) = maybe_event else {
                        tracing::debug!("Event sender dropped, waiting for session task to complete");
                        break session_task.await;
                    };
                    self.handle_dispatch_event(event);
                }
                maybe_client_message = self.context_rx.recv() => {
                    let Some(client_message) = maybe_client_message else {
                        panic!("context_tx is closed");
                    };
                    tokio::spawn(self.handle_client_message(client_message, handle.clone()));
                }
            }
        };
        let result = match session_task_result {
            Ok(Ok(value)) => Ok(value),
            Err(e) => Err(ClientError::JoinError(e)),
            Ok(Err(e)) => Err(e),
        };
        // Make sure that the lifetime extends until the end of the function
        // so that the drop guard isn't dropped before
        drop(cancellation_token_drop_guard);
        result
    }

    async fn handle_client_message(&mut self, msg: ClientMessage, session_handle: SessionHandle) {
        match msg {
            ClientMessage::RequestGuildCounts(request, result_tx, update_tx) => {
                if let Some(tx) = update_tx {
                    self.guild_counts_update_listeners
                        .insert(request.nonce.clone().unwrap(), tx);
                }
                let _ = result_tx.send(
                    session_handle
                        .send_message(OutgoingGatewayMessage::RequestGuildCounts(request))
                        .await,
                );
            }
        }
    }

    #[cfg(feature = "user_api")]
    #[tracing::instrument(skip(ctx))]
    async fn subscribe_to_everything(ctx: Context) -> Result<(), ClientError> {
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
