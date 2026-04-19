use std::{
    collections::HashMap,
    ops::{ControlFlow, Deref},
    sync::Arc,
    time::Duration,
};

use neptunium_cache_inmemory::{Cache, gateway::CachedDispatchEvent};
use neptunium_gateway::shard::{EventReceiveError, Shard, config::ShardConfig};
use neptunium_http::client::HttpClient;
use neptunium_model::gateway::{
    close_code::GatewayCloseCode,
    event::{dispatch::DispatchEvent, gateway::GatewayEvent, invalid_session::InvalidSessionEvent},
    payload::{
        incoming::GuildMembersChunk,
        outgoing::{
            ConnectionProperties, Heartbeat, LazyRequest, OutgoingGatewayMessage,
            PresenceUpdateOutgoing, RequestGuildMembers,
        },
    },
};
use tokio::{
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender, error::SendError, unbounded_channel},
        oneshot,
    },
    time::timeout,
};
use zeroize::Zeroizing;

use crate::{
    LIBRARY_NAME,
    client::error::{ClientErrorKind, Error},
    events::{EventError, EventHandler, context::Context},
};

pub mod api_info;
mod config;
pub mod error;
// pub mod http;
pub use config::*;

pub(crate) enum ClientMessage {
    ReceivedHeartbeatRequest,
    ScheduledHeartbeat,
    UpdatePresence(
        PresenceUpdateOutgoing,
        oneshot::Sender<Result<(), neptunium_gateway::shard::Error>>,
    ),
    RequestGuildMembers(
        RequestGuildMembers,
        oneshot::Sender<Result<(), neptunium_gateway::shard::Error>>,
        Option<UnboundedSender<GuildMembersChunk>>,
    ),
    SendLazyRequest(
        LazyRequest,
        UnboundedSender<Result<(), neptunium_gateway::shard::Error>>,
    ),
    PropagateEventError(EventError),
    LatencyMeasurement(oneshot::Sender<()>),
}

#[derive(Clone, Debug)]
struct ResumeInfo {
    session_id: Zeroizing<String>,
    // Doesn't appear to be a thing
    // resume_url: String,
}

#[expect(clippy::struct_excessive_bools)]
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
    guild_members_chunk_listeners: HashMap<String, UnboundedSender<GuildMembersChunk>>,
    expecting_heartbeat_ack: bool,
    expecting_heartbeat_ack_second_chance: bool,
    currently_resuming: bool,
    connection_process_timeout: Duration,
    identify_presence: Option<PresenceUpdateOutgoing>,
    send_identify_presence_on_reconnect: bool,
    gateway_retry_wait_time_fn: Box<dyn Fn(usize) -> Duration>,
    latency_measurements: Vec<oneshot::Sender<()>>,
    heartbeat_interval_override: Option<Duration>,
}

impl Deref for Client {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl Client {
    pub const USER_AGENT: &str = "Fluxer-Neptunium";
    const HEARTBEAT_SEND_TIMEOUT: Duration = Duration::from_mins(5);

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
            guild_members_chunk_listeners: HashMap::new(),
            expecting_heartbeat_ack: false,
            expecting_heartbeat_ack_second_chance: false,
            currently_resuming: false,
            connection_process_timeout: client_config.connection_process_timeout,
            identify_presence: client_config.initial_presence,
            send_identify_presence_on_reconnect: client_config
                .send_initial_presence_on_every_reconnect,
            gateway_retry_wait_time_fn: client_config.gateway_retry_wait_time_fn,
            latency_measurements: Vec::new(),
            heartbeat_interval_override: client_config.heartbeat_interval_override,
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
        let mut is_reconnect = false;
        let mut num_tries = 0;
        loop {
            let (got_past_connecting_tx, got_past_connecting_rx) = oneshot::channel();
            // Using Box::pin to avoid potentially causing a stack overflow by having a large future
            // (clippy lint large_futures)
            let result = Box::pin(self.inner_start(is_reconnect, got_past_connecting_tx)).await;
            if got_past_connecting_rx.await.is_ok() {
                if let Err(e) = &result
                    && let ClientErrorKind::SessionInvalidated = e.kind()
                {
                    num_tries += 1;
                } else {
                    num_tries = 0;
                }
            } else {
                num_tries += 1;
            }
            is_reconnect = true;
            let (tx, rx) = unbounded_channel();
            self.tx = tx;
            // drop old rx
            self.rx = rx;
            self.last_sequence_number = None;
            self.expecting_heartbeat_ack = false;
            self.expecting_heartbeat_ack_second_chance = false;
            let was_currently_resuming = self.currently_resuming;
            self.currently_resuming = false;
            if let Err(e) = self.shard.close().await {
                tracing::warn!("Error closing shard connection: {e}");
            }
            let error = match result {
                Ok(()) => {
                    tracing::debug!("Restarting client.");
                    continue;
                }
                Err(e) => e,
            };
            let reconnect_wait_time = (self.gateway_retry_wait_time_fn)(num_tries);
            if was_currently_resuming && let ClientErrorKind::SessionInvalidated = error.kind() {
                tracing::debug!(
                    "Session was not resumable. Immediately restarting client instead of waiting {} seconds.",
                    reconnect_wait_time.as_secs()
                );
                continue;
            }
            if self.auto_reconnect {
                if let ClientErrorKind::ConnectionClosed(close_frame) = error.kind()
                    && let Some(close_frame) = close_frame
                    && let Some(close_code) = GatewayCloseCode::from_u16(close_frame.code.into())
                    && !close_code.is_recoverable()
                {
                    tracing::debug!("Close code is not recoverable. Exiting.");
                    break Err(error);
                }
                tracing::warn!("Client error: {error}");
                tracing::info!(
                    "Reconnecting in {} seconds because auto-reconnect is enabled.",
                    reconnect_wait_time.as_secs()
                );
                tokio::time::sleep(reconnect_wait_time).await;
                continue;
            }
            tracing::debug!("Client error occured and auto-reconnect is disabled. Exiting.");
            break Err(error);
        }
    }

    #[expect(clippy::too_many_lines)]
    async fn inner_start(
        &mut self,
        is_reconnect: bool,
        got_past_connecting_tx: oneshot::Sender<()>,
    ) -> Result<(), self::error::Error> {
        tracing::debug!("Starting client...");
        let mut heartbeat_interval =
            // Box::pin to prevent large future on the stack
            match Box::pin(timeout(self.connection_process_timeout, self.connect(is_reconnect))).await {
                Ok(Ok(Ok(heartbeat_interval))) => heartbeat_interval,
                Ok(Ok(Err(()))) => {
                    tracing::debug!("Connection process triggered reconnect.");
                    return Ok(());
                }
                Ok(Err(e)) => {
                    tracing::debug!("Error in connection process.");
                    return Err(e);
                }
                Err(_) => {
                    tracing::debug!("Connection process timed out.");
                    return Err(Error::new(ClientErrorKind::TimedOut(
                        "connecting to gateway".to_owned(),
                    )));
                }
            };
        let _ = got_past_connecting_tx.send(());

        if let Some(heartbeat_interval_override) = self.heartbeat_interval_override {
            heartbeat_interval = heartbeat_interval_override;
        }

        loop {
            tokio::select! {
                message = self.rx.recv() => {
                    let Some(message) = message else {
                        tracing::debug!("Channel closed, exiting.");
                        return Ok(());
                    };
                    match message {
                        ClientMessage::ScheduledHeartbeat => {
                            if self.expecting_heartbeat_ack {
                                if self.expecting_heartbeat_ack_second_chance {
                                    tracing::error!("Expected heartbeat acknowledgement but none has been received after the heartbeat interval of {} ms. Restarting.", heartbeat_interval.as_millis());
                                    self.expecting_heartbeat_ack = false;
                                    self.expecting_heartbeat_ack_second_chance = false;
                                    return Err(Error::new(ClientErrorKind::TimedOut("receiving heartbeat acknowledgement".to_owned())));
                                }
                                tracing::warn!("Did not receive heartbeat acknowledgement.");
                                self.expecting_heartbeat_ack_second_chance = true;
                            }
                            tracing::trace!("Sending heartbeat.");
                            tokio::time::timeout(Self::HEARTBEAT_SEND_TIMEOUT, self.shard
                                .send_gateway_message(OutgoingGatewayMessage::Heartbeat(Heartbeat {
                                    last_sequence_number: self.last_sequence_number,
                                }))).await.map_err(|_| Error::new(ClientErrorKind::TimedOut("sending heartbeat".to_owned())))??;
                            self.expecting_heartbeat_ack = true;
                            // Heartbeats are a nice periodic event that happens where we can check for this stuff too
                            self.guild_members_chunk_listeners.retain(|_, tx| {
                                // tx being closed indicates that the function listening to tx has returned and has dropped
                                // the receiving end (meaning it has concluded that no more chunks will be sent)
                                !tx.is_closed()
                            });
                        }
                        ClientMessage::ReceivedHeartbeatRequest => {
                            self.expecting_heartbeat_ack = false;
                            self.expecting_heartbeat_ack_second_chance = false;
                            // Already indicates that the connection is still alive since the gateway is still sending stuff to the client.
                            tracing::trace!("expecting_heartbeat_ack and expecting_heartbeat_ack_second_chance are false now, because the gateway requested a heartbeat from us.");
                            tracing::trace!("Sending heartbeat due to request by gateway.");
                            tokio::time::timeout(Self::HEARTBEAT_SEND_TIMEOUT, self.shard
                                .send_gateway_message(OutgoingGatewayMessage::Heartbeat(Heartbeat {
                                    last_sequence_number: self.last_sequence_number,
                                }))).await.map_err(|_| Error::new(ClientErrorKind::TimedOut("sending heartbeat".to_owned())))??;
                        }
                        ClientMessage::UpdatePresence(data, sender) => {
                            let _ = sender.send(self.shard.send_gateway_message(OutgoingGatewayMessage::PresenceUpdate(data)).await);
                        }
                        ClientMessage::SendLazyRequest(data, sender) => {
                            let _ = sender.send(self.shard.send_gateway_message(OutgoingGatewayMessage::LazyRequest(data)).await);
                        }
                        ClientMessage::PropagateEventError(error) => {
                            // If an error occurs while closing, we still want to propagate the event error instead of the websocket error.
                            if let Err(e) = self.shard.close().await {
                                tracing::warn!("Error closing shard connection: {e}");
                            }
                            return Err(Error::new(ClientErrorKind::EventError(Box::new(error))));
                        }
                        ClientMessage::RequestGuildMembers(data, sender, tx) => {
                            if let Some(tx) = tx {
                                self.guild_members_chunk_listeners.insert(data.nonce.clone().unwrap(), tx);
                            }
                            let _ = sender.send(self.shard.send_gateway_message(OutgoingGatewayMessage::RequestGuildMembers(data)).await);
                        }
                        ClientMessage::LatencyMeasurement(tx) => {
                            self.latency_measurements.push(tx);
                            self.shard.send_gateway_message(OutgoingGatewayMessage::Heartbeat(Heartbeat { last_sequence_number: self.last_sequence_number })).await?;
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
                    if self.handle_event(message).await.map_err(|e| *e)?.is_break() {
                        return Ok(());
                    }
                }
            }
        }
    }

    /// Returns `Ok(Ok(...))` if connecting was successful, `Ok(Err(()))` if the client should restart,
    /// and `Err(...)` if an error occurred.
    async fn connect(&mut self, is_reconnect: bool) -> Result<Result<Duration, ()>, Error> {
        Ok(Ok(if let Some(resume_info) = self.resume_info.clone() {
            tracing::debug!("Waiting for `Hello` event from gateway.");
            let hello_event = match self.shard.next_event().await? {
                GatewayEvent::Hello(event) => event,
                event => {
                    return Err(Error::new(error::ClientErrorKind::UnexpectedEventReceived(
                        Box::new(event),
                    )));
                }
            };
            let mut heartbeat_interval: Duration = hello_event.heartbeat_interval.into();
            tracing::debug!(
                "Received Hello message from gateway. Heartbeat interval: {} ms. Now resuming.",
                heartbeat_interval.as_millis()
            );
            self.resume(resume_info).await?;

            tracing::debug!("Waiting for second `Hello` event from gateway.");
            let hello_event = if let Ok(event) =
                timeout(Duration::from_secs(5), self.shard.next_event()).await
            {
                match event? {
                    GatewayEvent::Hello(event) => Some(event),
                    other => {
                        if self.handle_event(other).await?.is_break() {
                            tracing::debug!(
                                "Handling other event while resuming triggered a restart. Clearing resume info."
                            );
                            self.resume_info = None;
                            return Ok(Err(()));
                        }
                        None
                    }
                }
            } else {
                tracing::debug!("Timed out waiting for another `Hello` from the gateway.");
                None
            };

            if let Some(hello_event) = hello_event {
                tracing::debug!("Received second `Hello` event from gateway.");
                heartbeat_interval = hello_event.heartbeat_interval.into();
            }

            tokio::spawn(Self::heartbeat_task(self.tx.clone(), heartbeat_interval));
            heartbeat_interval
        } else {
            tracing::debug!(
                "Starting new session instead of resuming because no resume info is present."
            );
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
                let _ = tx_clone.send(ClientMessage::ScheduledHeartbeat);
            });

            tokio::spawn(Self::heartbeat_task(self.tx.clone(), heartbeat_interval));

            self.shard
                .identify(
                    ConnectionProperties {
                        os: String::from(std::env::consts::OS),
                        browser: String::from(LIBRARY_NAME),
                        device: String::from(LIBRARY_NAME),
                    },
                    if is_reconnect && !self.send_identify_presence_on_reconnect {
                        None
                    } else {
                        self.identify_presence.clone()
                    },
                )
                .await?;
            heartbeat_interval
        }))
    }

    async fn heartbeat_task(tx: UnboundedSender<ClientMessage>, heartbeat_interval: Duration) {
        loop {
            tokio::time::sleep(heartbeat_interval).await;
            if tx.send(ClientMessage::ScheduledHeartbeat).is_err() {
                // The message receiver has stopped, the client is likely restarting and will
                // respawn this task later. (Either way, stop this task as it is useless when it
                // can't send messages.)
                tracing::debug!("Heartbeat task stopping due to mpsc channel being closed.");
                break;
            }
        }
    }

    /// Returns `ControlFlow::Break(())` if the client should reconnect, and `ControlFlow::Continue(())` if the client
    /// should wait for the next event (continue normally).
    async fn handle_event(
        &mut self,
        event: GatewayEvent,
    ) -> Result<ControlFlow<()>, Box<self::error::Error>> {
        match event {
            GatewayEvent::Heartbeat => {
                let _ = self.tx.send(ClientMessage::ReceivedHeartbeatRequest);
            }
            GatewayEvent::HeartbeatAck => {
                // Complete all pending latency measurements
                self.latency_measurements.drain(..).for_each(|tx| {
                    let _ = tx.send(());
                });
                self.expecting_heartbeat_ack = false;
                self.expecting_heartbeat_ack_second_chance = false;
                tracing::trace!(
                    "expecting_heartbeat_ack and expecting_heartbeat_ack_second_chance are false now."
                );
            }
            GatewayEvent::Hello(hello) => {
                tracing::warn!("Received `Hello` event more than one time. Received: {hello:?}");
            }
            GatewayEvent::InvalidSession(InvalidSessionEvent { resumable }) => {
                if resumable {
                    tracing::debug!(
                        "Resuming after gateway has sent a `InvalidSession` gateway event with resumable set to `true`."
                    );
                    if let Some(resume_info) = self.resume_info.clone() {
                        self.resume(resume_info).await?;
                    } else {
                        return Ok(ControlFlow::Break(()));
                    }
                } else {
                    // When resuming, the gateway may send InvalidSession if we didn't reconnect in time to resume.
                    // To prevent an infinite resume loop, we clear the resume_info here. It's also generally a good idea
                    // to assume that when the gateway tells us that we may not resume, we should clear the resume info.
                    self.resume_info = None;
                    return Err(Box::new(Error::new(ClientErrorKind::SessionInvalidated)));
                }
            }
            GatewayEvent::Reconnect => {
                tracing::debug!("Resuming after gateway has sent a `Reconnect` gateway event.");
                if let Some(resume_info) = self.resume_info.clone() {
                    self.resume(resume_info).await?;
                } else {
                    return Ok(ControlFlow::Break(()));
                }
            }
            GatewayEvent::Dispatch(payload) => {
                // TODO: Maybe check if the current sequence number is bigger than the received one because that shouldn't happen? Or something...
                self.last_sequence_number = Some(payload.sequence_number);
                self.handle_dispatch_event(payload.event);
                return Ok(ControlFlow::Continue(()));
            }
            GatewayEvent::GatewayError(payload) => {
                tracing::warn!("Gateway error: {:?}", payload);
                return Ok(ControlFlow::Continue(()));
            }
        }
        Ok(ControlFlow::Continue(()))
    }

    async fn resume(&mut self, resume_info: ResumeInfo) -> Result<(), Box<Error>> {
        tracing::debug!("Resuming from previous session.");
        self.currently_resuming = true;
        self.shard
            .resume(
                resume_info.session_id,
                self.last_sequence_number.unwrap_or(0),
            )
            .await
            .map_err(|e| Box::new(Error::new(ClientErrorKind::NetworkError(e))))
    }

    #[expect(clippy::too_many_lines)]
    fn handle_dispatch_event(&mut self, event: DispatchEvent) {
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
        macro_rules! call_event_handlers_noarc {
            ($always_propagate_event_errors:expr, $tx:expr, $handlers:expr, $ctx:expr, $data:expr => $func_name:ident) => {{
                let always_propagate_event_errors = $always_propagate_event_errors;
                for handler in &$handlers {
                    let handler = Arc::clone(handler);
                    let cloned_data = $data.clone();
                    let ctx_clone = $ctx.clone();
                    let tx_clone = $tx.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handler.$func_name(ctx_clone, cloned_data).await {
                            if e.propagate || always_propagate_event_errors {
                                // Discarding the error because this task returns anway.
                                let _ = tx_clone
                                    .send($crate::client::ClientMessage::PropagateEventError(e));
                            } else {
                                tracing::warn!("Event handler returned error: {e}");
                            }
                        }
                    });
                }
            }};
        }

        let event = CachedDispatchEvent::from_dispatch_event(event, &self.context.cache);

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
                call_event_handlers_noarc!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_update);
            }
            CachedDispatchEvent::UserPinnedDmsUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_pinned_dms_update);
            }
            CachedDispatchEvent::UserSettingsUpdate(data) => {
                call_event_handlers_noarc!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_user_settings_update);
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
                call_event_handlers_noarc!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_saved_message_create);
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
                call_event_handlers_noarc!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_update);
            }
            CachedDispatchEvent::GuildSync(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_sync);
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
                call_event_handlers_noarc!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_role_create);
            }
            CachedDispatchEvent::GuildRoleUpdate(data) => {
                call_event_handlers_noarc!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_role_update);
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
                call_event_handlers_noarc!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_create);
            }
            CachedDispatchEvent::ChannelUpdate(data) => {
                call_event_handlers_noarc!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_channel_update);
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
                call_event_handlers_noarc!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_message_update);
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
            CachedDispatchEvent::GuildMembersChunk(data) => {
                // Unfortunately it seems that there is no easy way to avoid cloning the nonce here,
                // else rust will give an error that part of `data` is borrowed.
                if let Some(nonce) = data.nonce.clone()
                    && let Some(listener_tx) = self.guild_members_chunk_listeners.get(&nonce)
                {
                    // The receiving end being dropped indicates that it has received all the messages it needs.
                    // However, receiving the same nonce is concerning as it likely indicates a bug with how it
                    // detects whether all messages have been received.
                    if let Err(SendError(data)) = listener_tx.send(data) {
                        tracing::warn!(
                            "Guild member chunk with nonce \"{}\" is managed but the receiver has been dropped.",
                            nonce
                        );
                        self.guild_members_chunk_listeners.remove(&nonce);
                        call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_members_chunk);
                    }
                } else {
                    call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_members_chunk);
                }
            }
            CachedDispatchEvent::GuildMemberListUpdate(data) => {
                call_event_handlers!(self.always_propagate_event_errors, self.tx, self.event_handlers, self.context, data => on_guild_member_list_update);
            }
        }
    }
}
