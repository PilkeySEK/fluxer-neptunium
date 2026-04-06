use std::{ops::Deref, sync::Arc, time::Duration};

use neptunium_cache_inmemory::{Cache, gateway::CachedDispatchEvent};
use neptunium_gateway::shard::{EventReceiveError, Shard, config::ShardConfig};
use neptunium_http::client::HttpClient;
use neptunium_model::gateway::{
    event::{dispatch::DispatchEvent, gateway::GatewayEvent, invalid_session::InvalidSessionEvent},
    payload::outgoing::{
        ConnectionProperties, Heartbeat, OutgoingGatewayMessage, PresenceUpdateOutgoing,
    },
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use zeroize::Zeroizing;

use crate::{
    client::error::{ClientErrorKind, Error},
    events::{EventError, EventHandler, context::Context},
};

pub mod api_info;
mod config;
pub mod error;
// pub mod http;
pub use config::*;

pub(crate) enum ClientMessage {
    Heartbeat,
    UpdatePresence(PresenceUpdateOutgoing),
    PropagateEventError(EventError),
}

#[derive(Clone, Debug)]
struct ResumeInfo {
    session_id: Zeroizing<String>,
    // Doesn't appear to be a thing
    // resume_url: String,
}

pub struct Client {
    shard: Shard,
    event_handlers: Vec<Arc<dyn EventHandler + Sync + 'static>>,
    last_sequence_number: Option<u64>,
    context: Context,
    tx: UnboundedSender<ClientMessage>,
    rx: UnboundedReceiver<ClientMessage>,
    always_propagate_event_errors: bool,
    resume_info: Option<ResumeInfo>,
    auto_reconnect: bool,
    auto_reconnect_wait_time: Duration,
}

impl Deref for Client {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl Client {
    pub const USER_AGENT: &str = "Fluxer-Neptunium";

    /// Create a new client, given a `ShardConfig` or token.
    /// # Examples
    /// ```no_run
    /// # use fluxer_neptunium::prelude::*;
    /// # fn main() {
    /// let token: String = std::env::var("FLUXER_TOKEN").unwrap();
    /// let client = Client::new(token);
    /// // The `ShardConfigBuilder` allows for other configuration:
    /// let token: String = std::env::var("FLUXER_TOKEN").unwrap();
    /// let client = Client::new(
    ///     ShardConfig::builder()
    ///         .token(token)
    ///         .ignored_events(GatewayEventFlags::GUILDS)
    ///         .build(),
    /// );
    /// # }
    /// ```
    #[must_use]
    pub fn new(shard_config: impl Into<ShardConfig>) -> Self {
        Self::new_with_config(shard_config, ClientConfig::default())
    }

    #[must_use]
    pub fn new_with_config(
        shard_config: impl Into<ShardConfig>,
        client_config: ClientConfig,
    ) -> Self {
        let shard_config = shard_config.into();
        let token_clone = (*shard_config.token).clone();
        #[cfg(not(feature = "user_api"))]
        let mut api_client = HttpClient::new(token_clone);
        #[cfg(feature = "user_api")]
        let mut api_client = HttpClient::new(token_clone, client_config.token_type);

        if let Some(api_base_url) = client_config.api_base_url {
            api_client.set_api_base_url(api_base_url);
        }
        api_client.set_user_agent(format!("{}/{}", Self::USER_AGENT, crate::VERSION));

        let (tx, rx) = unbounded_channel::<ClientMessage>();

        Self {
            shard: Shard::new(shard_config),
            event_handlers: Vec::new(),
            last_sequence_number: None,
            context: Context {
                http_client: Arc::new(api_client),
                tx: tx.clone(),
                cache: Arc::new(Cache::new(client_config.cache_config)),
            },
            tx,
            rx,
            always_propagate_event_errors: client_config.always_propagate_event_errors,
            resume_info: None,
            auto_reconnect: client_config.auto_reconnect,
            auto_reconnect_wait_time: client_config.auto_reconnect_wait_time,
        }
    }

    #[must_use]
    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn register_event_handler(&mut self, handler: impl EventHandler + Sync + 'static) {
        self.event_handlers
            .push(Arc::new(handler) as Arc<dyn EventHandler + Sync>);
    }

    /// Start the client. This will run indefinetly unless the connection is closed or some other
    /// error occurs.
    ///
    /// If auto-reconnect is enabled (the default), the client will never return and instead always try reconnecting after the
    /// configured auto reconnect wait time (the default is 30 seconds) if an error occurs.
    /// # Errors
    /// Returns an error if unexpected data is received, or if a network error occurs.
    pub async fn start(&mut self) -> Result<(), self::error::Error> {
        loop {
            // Using Box::pin to avoid potentially causing a stack overflow by having a large future
            // (clippy lint large_futures)
            let result = Box::pin(self.inner_start()).await;
            let error = match result {
                Ok(result) => return Ok(result),
                Err(e) => e,
            };
            if self.auto_reconnect {
                tracing::warn!("Client error: {error}");
                tracing::info!(
                    "Reconnecting in {} seconds because auto-reconnect is enabled.",
                    self.auto_reconnect_wait_time.as_secs()
                );
                if let Err(e) = self.shard.close().await {
                    tracing::warn!("Error closing shard connection: {e}");
                }
                tokio::time::sleep(self.auto_reconnect_wait_time).await;
                continue;
            }
            tracing::debug!("Client error occured and auto-reconnect is disabled. Returning.");
            if let Err(e) = self.shard.close().await {
                tracing::warn!("Error closing shard connection: {e}");
            }
            break Err(error);
        }
    }

    async fn inner_start(&mut self) -> Result<(), self::error::Error> {
        tracing::info!("Starting client...");
        let hello_event = match self.shard.next_event().await? {
            GatewayEvent::Hello(event) => event,
            event => {
                return Err(Error::new(error::ClientErrorKind::UnexpectedEventReceived(
                    Box::new(event),
                )));
            }
        };

        let heartbeat_interval: Duration = hello_event.heartbeat_interval.into();

        tracing::debug!(
            "Received Hello message from gateway. Heartbeat interval: {} ms",
            heartbeat_interval.as_millis()
        );

        let tx_clone = self.tx.clone();
        tokio::spawn(async move {
            #[expect(clippy::cast_possible_truncation)]
            let millis = rand::random_range(0..heartbeat_interval.as_millis() as u64);
            tokio::time::sleep(Duration::from_millis(millis)).await;
            let _ = tx_clone.send(ClientMessage::Heartbeat);
        });

        let tx_clone = self.tx.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(heartbeat_interval).await;
                if tx_clone.send(ClientMessage::Heartbeat).is_err() {
                    // The message receiver has stopped
                    tracing::debug!("Heartbeat task stopping due to channel being closed.");
                    break;
                }
            }
        });

        self.shard
            .identify(ConnectionProperties {
                os: String::from(std::env::consts::OS),
                browser: String::from("fluxer-neptunium"),
                device: String::from("fluxer-neptunium"),
            })
            .await?;

        loop {
            tokio::select! {
                message = self.rx.recv() => {
                    let Some(message) = message else {
                        tracing::debug!("Channel closed, exiting.");
                        return Ok(());
                    };
                    match message {
                        ClientMessage::Heartbeat => {
                            tracing::trace!("Sending heartbeat.");
                            self.shard
                                .send_gateway_message(OutgoingGatewayMessage::Heartbeat(Heartbeat {
                                    last_sequence_number: self.last_sequence_number,
                                }))
                                .await?;
                        }
                        ClientMessage::UpdatePresence(data) => {
                            self.shard.send_gateway_message(OutgoingGatewayMessage::PresenceUpdate(data)).await?;
                        }
                        ClientMessage::PropagateEventError(error) => {
                            // If an error occurs while closing, we still want to propagate the event error instead of the websocket error.
                            if let Err(e) = self.shard.close().await {
                                tracing::warn!("Error closing shard connection: {e}");
                            }
                            return Err(Error::new(ClientErrorKind::EventError(Box::new(error))));
                        }
                    }
                },
                message = self.shard.next_event() => {
                    let message = match message {
                        Ok(message) => message,
                        Err(EventReceiveError::ParseError(e)) => {
                            tracing::warn!("Failed to parse at `{}`: {}", e.path(), e);
                            continue;
                        }
                        Err(EventReceiveError::TungsteniteError(e)) => {
                            tracing::error!("Network Error: {}", e);
                            return Err(Error::new(error::ClientErrorKind::NetworkError(e)));
                        }
                        Err(EventReceiveError::UnsupportedMessageEncoding) => {
                            tracing::error!("Unsupported message encoding, can't continue.");
                            return Err(Error::new(error::ClientErrorKind::UnsupportedMessageEncoding));
                        }
                        Err(EventReceiveError::Closed(frame)) => {
                            let error = Error::new(error::ClientErrorKind::ConnectionClosed(frame));
                            tracing::debug!("{error}");
                            return Err(error);
                        }
                    };
                    tracing::trace!("Received message: {message:?}");
                    self.handle_event(message).await.map_err(|e| *e)?;
                }
            }
        }
    }

    async fn handle_event(&mut self, event: GatewayEvent) -> Result<(), Box<self::error::Error>> {
        match event {
            GatewayEvent::Heartbeat => {
                let _ = self.tx.send(ClientMessage::Heartbeat);
            }
            GatewayEvent::HeartbeatAck => {}
            GatewayEvent::Hello(hello) => {
                tracing::warn!("Received `Hello` event more than one time. Received: {hello:?}");
            }
            GatewayEvent::InvalidSession(InvalidSessionEvent { resumable }) => {
                if resumable {
                    tracing::debug!(
                        "Resuming after gateway has sent a `InvalidSession` gateway event with resumable set to `true`."
                    );
                    self.resume().await?;
                } else {
                    return Err(Box::new(Error::new(ClientErrorKind::SessionInvalidated)));
                }
            }
            GatewayEvent::Reconnect => {
                tracing::debug!("Resuming after gateway has sent a `Reconnect` gateway event.");
                self.resume().await?;
            }
            GatewayEvent::Dispatch(payload) => {
                // TODO: Maybe check if the current sequence number is bigger than the received one because that shouldn't happen? Or something...
                self.last_sequence_number = Some(payload.sequence_number);
                return self.handle_dispatch_event(payload.event).await;
            }
        }
        Ok(())
    }

    async fn resume(&mut self) -> Result<(), Box<Error>> {
        let Some(resume_info) = &self.resume_info else {
            tracing::warn!("Resuming was requested but there is no resume info.");
            return Err(Box::new(Error::new(
                ClientErrorKind::UnexpectedEventReceived(Box::new(GatewayEvent::Reconnect)),
            )));
        };
        self.shard
            .resume(
                resume_info.session_id.clone(),
                self.last_sequence_number.unwrap_or(0),
            )
            .await
            .map_err(|e| Box::new(Error::new(ClientErrorKind::NetworkError(e))))
    }

    #[expect(clippy::too_many_lines)]
    async fn handle_dispatch_event(
        &mut self,
        event: DispatchEvent,
    ) -> Result<(), Box<self::error::Error>> {
        tracing::trace!("Dispatch Event: {event:?}");
        macro_rules! call_event_handlers {
            ($always_propagate_event_errors:expr, $tx:expr, $handlers:expr, $ctx:expr, $data:expr => $func_name:ident) => {{
                let arc = Arc::new($data);
                let always_propagate_event_errors = $always_propagate_event_errors;
                for handler in &$handlers {
                    let handler = Arc::clone(handler);
                    let cloned_arc = Arc::clone(&arc);
                    let ctx_clone = $ctx.clone();
                    let tx_clone = $tx.clone();
                    // TODO: Maybe store all the `JoinHandle`s in an array in the `Client` struct so that they could all be cancelled
                    // when the `Client` stops? Maybe...
                    tokio::spawn(async move {
                        if let Err(e) = handler.$func_name(ctx_clone, cloned_arc).await {
                            if e.propagate || always_propagate_event_errors {
                                // Discarding the error because this task returns anway.
                                let _ = tx_clone.send($crate::client::ClientMessage::PropagateEventError(e));
                            } else {
                                tracing::warn!("Event handler returned error: {e}");
                            }
                        }
                    });
                }
            }};
        }

        let event = CachedDispatchEvent::from_dispatch_event(event, &self.context.cache).await;

        match event {
            CachedDispatchEvent::Ready(data) => {
                self.resume_info = Some(ResumeInfo {
                    session_id: data.session_id.clone(),
                });
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_ready);
            }
            CachedDispatchEvent::Resumed(()) => {
                for handler in &self.event_handlers {
                    let handler = Arc::clone(handler);
                    let ctx_clone = self.context.clone();
                    let tx_clone = self.tx.clone();
                    let always_propagate_event_errors = self.always_propagate_event_errors;
                    tokio::spawn(async move {
                        if let Err(e) = handler.on_resumed(ctx_clone).await {
                            if e.propagate || always_propagate_event_errors {
                                let _ = tx_clone.send(ClientMessage::PropagateEventError(e));
                            } else {
                                tracing::warn!("Event handler returned error: {e}");
                            }
                        }
                    });
                }
            }
            CachedDispatchEvent::SessionsReplace(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_sessions_replace);
            }
            CachedDispatchEvent::GuildAuditLogEntryCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_audit_log_entry_create);
            }
            CachedDispatchEvent::UserUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_update);
            }
            CachedDispatchEvent::UserPinnedDmsUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_pinned_dms_update);
            }
            CachedDispatchEvent::UserSettingsUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_settings_update);
            }
            CachedDispatchEvent::UserGuildSettingsUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_guild_settings_update);
            }
            CachedDispatchEvent::UserNoteUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_note_update);
            }
            CachedDispatchEvent::RecentMentionDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_recent_mention_delete);
            }
            CachedDispatchEvent::SavedMessageCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_saved_message_create);
            }
            CachedDispatchEvent::SavedMessageDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_saved_message_delete);
            }
            CachedDispatchEvent::FavoriteMemeCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_favorite_meme_create);
            }
            CachedDispatchEvent::FavoriteMemeUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_favorite_meme_update);
            }
            CachedDispatchEvent::FavoriteMemeDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_favorite_meme_delete);
            }
            CachedDispatchEvent::AuthSessionChange(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_auth_session_change);
            }
            CachedDispatchEvent::PresenceUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_presence_update);
            }
            CachedDispatchEvent::GuildCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_create);
            }
            CachedDispatchEvent::GuildUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_update);
            }
            CachedDispatchEvent::GuildDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_delete);
            }
            CachedDispatchEvent::GuildMemberAdd(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_member_add);
            }
            CachedDispatchEvent::GuildMemberUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_member_update);
            }
            CachedDispatchEvent::GuildMemberRemove(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_member_remove);
            }
            CachedDispatchEvent::GuildRoleCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_role_create);
            }
            CachedDispatchEvent::GuildRoleUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_role_update);
            }
            CachedDispatchEvent::GuildRoleUpdateBulk(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_role_update_bulk);
            }
            CachedDispatchEvent::GuildRoleDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_role_delete);
            }
            CachedDispatchEvent::GuildEmojisUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_emojis_update);
            }
            CachedDispatchEvent::GuildStickersUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_stickers_update);
            }
            CachedDispatchEvent::GuildBanAdd(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_ban_add);
            }
            CachedDispatchEvent::GuildBanRemove(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_ban_remove);
            }
            CachedDispatchEvent::ChannelCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_create);
            }
            CachedDispatchEvent::ChannelUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_update);
            }
            CachedDispatchEvent::ChannelUpdateBulk(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_update_bulk);
            }
            CachedDispatchEvent::ChannelDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_delete);
            }
            CachedDispatchEvent::ChannelPinsUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_pins_update);
            }
            CachedDispatchEvent::ChannelPinsAck(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_pins_ack);
            }
            CachedDispatchEvent::ChannelRecipientAdd(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_recipient_add);
            }
            CachedDispatchEvent::ChannelRecipientRemove(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_recipient_remove);
            }
            CachedDispatchEvent::MessageCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_create);
            }
            CachedDispatchEvent::MessageUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_update);
            }
            CachedDispatchEvent::MessageDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_delete);
            }
            CachedDispatchEvent::MessageDeleteBulk(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_delete_bulk);
            }
            CachedDispatchEvent::MessageReactionAdd(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_reaction_add);
            }
            CachedDispatchEvent::MessageReactionRemove(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_reaction_remove);
            }
            CachedDispatchEvent::MessageReactionRemoveAll(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_reaction_remove_all);
            }
            CachedDispatchEvent::MessageReactionRemoveEmoji(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_reaction_remove_emoji);
            }
            CachedDispatchEvent::MessageAck(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_ack);
            }
            CachedDispatchEvent::TypingStart(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_typing_start);
            }
            CachedDispatchEvent::WebhooksUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_webhooks_update);
            }
            CachedDispatchEvent::InviteCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_invite_create);
            }
            CachedDispatchEvent::InviteDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_invite_delete);
            }
            CachedDispatchEvent::RelationshipAdd(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_relationship_add);
            }
            CachedDispatchEvent::RelationshipUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_relationship_update);
            }
            CachedDispatchEvent::RelationshipRemove(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_relationship_remove);
            }
            CachedDispatchEvent::VoiceStateUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_voice_state_update);
            }
            CachedDispatchEvent::VoiceServerUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_voice_server_update);
            }
            CachedDispatchEvent::CallCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_call_create);
            }
            CachedDispatchEvent::CallUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_call_update);
            }
            CachedDispatchEvent::CallDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_call_delete);
            }
            // #[cfg(feature = "user_api")]
            CachedDispatchEvent::PassiveUpdates(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_passive_updates);
            }
        }
        Ok(())
    }
}
