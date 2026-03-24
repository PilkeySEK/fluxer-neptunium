use std::{ops::Deref, sync::Arc, time::Duration};

use neptunium_gateway::shard::{EventReceiveError, Shard, config::ShardConfig};
use neptunium_http::client::HttpClient;
use neptunium_model::gateway::{
    event::{dispatch::DispatchEvent, gateway::GatewayEvent, invalid_session::InvalidSessionEvent},
    payload::outgoing::{
        OutgoingGatewayMessage, heartbeat::Heartbeat, identify::ConnectionProperties,
        presence_update::PresenceUpdateOutgoing,
    },
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

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

pub struct Client {
    shard: Shard,
    event_handlers: Vec<Arc<dyn EventHandler + Sync + 'static>>,
    last_sequence_number: Option<u64>,
    context: Context,
    tx: UnboundedSender<ClientMessage>,
    rx: UnboundedReceiver<ClientMessage>,
    always_propagate_event_errors: bool,
}

impl Deref for Client {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl Client {
    pub const USER_AGENT: &str = "Fluxer-Neptunium";

    #[must_use]
    pub fn new(shard_config: ShardConfig) -> Self {
        Self::new_with_config(shard_config, ClientConfig::default())
    }

    #[must_use]
    pub fn new_with_config(shard_config: ShardConfig, client_config: ClientConfig) -> Self {
        let token_clone = (*shard_config.token).clone();
        let mut api_client = HttpClient::new(token_clone);
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
            },
            tx,
            rx,
            always_propagate_event_errors: client_config.always_propagate_event_errors,
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
    /// # Errors
    /// Returns an error if unexpected data is received, or if a network error occurs.
    pub async fn start(&mut self) -> Result<(), self::error::Error> {
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
                            tracing::debug!("Sending heartbeat.");
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
                                tracing::warn!("Error closing shard: {e}");
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
                    self.handle_event(message).map_err(|e| *e)?;
                }
            }
        }
    }

    /// Does not block because it spawns a new task for each event handler.
    fn handle_event(&mut self, event: GatewayEvent) -> Result<(), Box<self::error::Error>> {
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
                    todo!(
                        "Session was invalidated and resumable is set to true, but this has not been implemented yet."
                    )
                } else {
                    return Err(Box::new(Error::new(ClientErrorKind::SessionInvalidated)));
                }
            }
            GatewayEvent::Reconnect => todo!("Reconnecting is not yet implemented"),
            GatewayEvent::Dispatch(payload) => {
                // TODO: Maybe check if the current sequence number is bigger than the received one because that shouldn't happen? Or something...
                self.last_sequence_number = Some(payload.sequence_number);
                return self.handle_dispatch_event(payload.event);
            }
        }
        Ok(())
    }

    /// Does not block because it spawns a new task for each event handler.
    #[expect(clippy::too_many_lines, clippy::unnecessary_wraps)]
    fn handle_dispatch_event(
        &mut self,
        event: DispatchEvent,
    ) -> Result<(), Box<self::error::Error>> {
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

        match event {
            DispatchEvent::Ready(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_ready);
            }
            DispatchEvent::Resumed(()) => {
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
            DispatchEvent::SessionsReplace(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_sessions_replace);
            }
            DispatchEvent::GuildAuditLogEntryCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_audit_log_entry_create);
            }
            DispatchEvent::UserUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_update);
            }
            DispatchEvent::UserPinnedDmsUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_pinned_dms_update);
            }
            DispatchEvent::UserSettingsUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_settings_update);
            }
            DispatchEvent::UserGuildSettingsUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_guild_settings_update);
            }
            DispatchEvent::UserNoteUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_note_update);
            }
            DispatchEvent::RecentMentionDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_recent_mention_delete);
            }
            DispatchEvent::SavedMessageCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_saved_message_create);
            }
            DispatchEvent::SavedMessageDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_saved_message_delete);
            }
            DispatchEvent::FavoriteMemeCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_favorite_meme_create);
            }
            DispatchEvent::FavoriteMemeUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_favorite_meme_update);
            }
            DispatchEvent::FavoriteMemeDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_favorite_meme_delete);
            }
            DispatchEvent::AuthSessionChange(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_auth_session_change);
            }
            DispatchEvent::PresenceUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_presence_update);
            }
            DispatchEvent::GuildCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_create);
            }
            DispatchEvent::GuildUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_update);
            }
            DispatchEvent::GuildDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_delete);
            }
            DispatchEvent::GuildMemberAdd(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_member_add);
            }
            DispatchEvent::GuildMemberUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_member_update);
            }
            DispatchEvent::GuildMemberRemove(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_member_remove);
            }
            DispatchEvent::GuildRoleCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_role_create);
            }
            DispatchEvent::GuildRoleUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_role_update);
            }
            DispatchEvent::GuildRoleUpdateBulk(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_role_update_bulk);
            }
            DispatchEvent::GuildRoleDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_role_delete);
            }
            DispatchEvent::GuildEmojisUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_emojis_update);
            }
            DispatchEvent::GuildStickersUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_stickers_update);
            }
            DispatchEvent::GuildBanAdd(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_ban_add);
            }
            DispatchEvent::GuildBanRemove(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_ban_remove);
            }
            DispatchEvent::ChannelCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_create);
            }
            DispatchEvent::ChannelUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_update);
            }
            DispatchEvent::ChannelUpdateBulk(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_update_bulk);
            }
            DispatchEvent::ChannelDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_delete);
            }
            DispatchEvent::ChannelPinsUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_pins_update);
            }
            DispatchEvent::ChannelPinsAck(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_pins_ack);
            }
            DispatchEvent::ChannelRecipientAdd(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_recipient_add);
            }
            DispatchEvent::ChannelRecipientRemove(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_recipient_remove);
            }
            DispatchEvent::MessageCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_create);
            }
            DispatchEvent::MessageUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_update);
            }
            DispatchEvent::MessageDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_delete);
            }
            DispatchEvent::MessageDeleteBulk(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_delete_bulk);
            }
            DispatchEvent::MessageReactionAdd(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_reaction_add);
            }
            DispatchEvent::MessageReactionRemove(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_reaction_remove);
            }
            DispatchEvent::MessageReactionRemoveAll(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_reaction_remove_all);
            }
            DispatchEvent::MessageReactionRemoveEmoji(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_reaction_remove_emoji);
            }
            DispatchEvent::MessageAck(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_ack);
            }
            DispatchEvent::TypingStart(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_typing_start);
            }
            DispatchEvent::WebhooksUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_webhooks_update);
            }
            DispatchEvent::InviteCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_invite_create);
            }
            DispatchEvent::InviteDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_invite_delete);
            }
            DispatchEvent::RelationshipAdd(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_relationship_add);
            }
            DispatchEvent::RelationshipUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_relationship_update);
            }
            DispatchEvent::RelationshipRemove(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_relationship_remove);
            }
            DispatchEvent::VoiceStateUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_voice_state_update);
            }
            DispatchEvent::VoiceServerUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_voice_server_update);
            }
            DispatchEvent::CallCreate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_call_create);
            }
            DispatchEvent::CallUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_call_update);
            }
            DispatchEvent::CallDelete(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_call_delete);
            }
        }
        Ok(())
    }
}
