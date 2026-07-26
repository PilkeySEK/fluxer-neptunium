use std::{
    collections::HashMap,
    ops::ControlFlow,
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};

use bon::Builder;
use neptunium_cache_inmemory::{Cache, gateway::cached_payload::CachedGuildMembersChunk};
use neptunium_gateway::shard::{Shard, config::ShardConfig};
use neptunium_http::client::HttpClient;
use neptunium_model::gateway::{
    close_code::GatewayCloseCode,
    event::{dispatch::DispatchEvent, gateway::GatewayEventIncoming},
    payload::{
        incoming::GuildCountsUpdate,
        outgoing::{
            ConnectionProperties, Heartbeat, LazyRequest, OutgoingGatewayMessage,
            PresenceUpdateOutgoing, RequestGuildCounts, RequestGuildMembers,
        },
    },
};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    oneshot,
};
use zeroize::Zeroizing;

mod config;
mod dispatch_event_impl;
pub mod error;
pub use config::*;

use crate::{
    client::error::{ClientErrorKind, Error},
    events::{EventHandler, context::Context},
};

enum ShardTaskMessage {
    Send(OutgoingGatewayMessage),
    SendWithResultOneshot(
        OutgoingGatewayMessage,
        oneshot::Sender<Result<(), tokio_tungstenite::tungstenite::Error>>,
    ),
    SendWithResultUnbounded(
        OutgoingGatewayMessage,
        UnboundedSender<Result<(), tokio_tungstenite::tungstenite::Error>>,
    ),
}

pub(crate) enum ClientMessage {
    UpdatePresence(
        PresenceUpdateOutgoing,
        oneshot::Sender<Result<(), neptunium_gateway::shard::Error>>,
    ),
    RequestGuildMembers(
        RequestGuildMembers,
        oneshot::Sender<Result<(), neptunium_gateway::shard::Error>>,
        Option<UnboundedSender<CachedGuildMembersChunk>>,
    ),
    SendLazyRequest(
        LazyRequest,
        UnboundedSender<Result<(), neptunium_gateway::shard::Error>>,
    ),
    // PropagateEventError(EventError),
    LatencyMeasurement(oneshot::Sender<()>),
    RequestGuildCounts(
        RequestGuildCounts,
        oneshot::Sender<Result<(), neptunium_gateway::shard::Error>>,
        Option<oneshot::Sender<GuildCountsUpdate>>,
    ),
    GracefullyStop,
}

struct SessionInfo<'a> {
    last_sequence_number: &'a mut Option<u64>,
    last_heartbeat_ack_at: &'a mut SystemTime,
    no_heartbeat_ack_time_limit: Duration,
    shard_task_tx: &'a UnboundedSender<ShardTaskMessage>,
    session_tx: &'a UnboundedSender<ClientSessionMessage>,
    already_sent_presence_in_identify: &'a mut bool,
    resume_info: &'a mut Option<ResumeInfo>,
    queued_dispatch_events: &'a mut Vec<DispatchEvent>,
    ready_received_yet: &'a mut bool,
}

enum ClientSessionMessage {
    SendHeartbeat,
    GatewayEvent(Box<GatewayEventIncoming>),
    ShardTaskEnd(Option<Error>),
}

#[derive(Builder, Clone, Debug)]
pub struct ResumeInfo {
    #[builder(into)]
    pub session_id: Zeroizing<String>,
    pub last_sequence_number: u64,
}

pub struct Client {
    shard_config: ShardConfig,
    event_handlers: Vec<Arc<dyn EventHandler + Sync + 'static>>,
    context: Context,
    // tx: UnboundedSender<ClientMessage>,
    rx: UnboundedReceiver<ClientMessage>,
    gateway_retry_wait_time_fn: Box<dyn Fn(usize) -> Duration + Send + Sync>,
    send_identify_presence_on_every_reconnect: bool,
    identify_presence: Option<PresenceUpdateOutgoing>,
    guild_members_chunk_listeners: HashMap<String, UnboundedSender<CachedGuildMembersChunk>>,
    guild_counts_update_listeners: HashMap<String, oneshot::Sender<GuildCountsUpdate>>,
    latency_measurements: Vec<oneshot::Sender<()>>,
    initial_resume_info: Option<ResumeInfo>,
    #[cfg(feature = "user_api")]
    subscribe_to_everything: bool,
}

impl std::ops::Deref for Client {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl Client {
    #[must_use]
    pub fn new(shard_config: impl Into<ShardConfig>) -> Self {
        Self::new_with_config(shard_config, ClientConfig::default())
    }

    #[must_use]
    pub fn new_with_config(
        shard_config: impl Into<ShardConfig>,
        client_config: ClientConfig,
    ) -> Self {
        let mut shard_config = shard_config.into();

        if shard_config.send_timeout.is_none() && client_config.overwrite_send_timeout {
            // 2 minutes is probably very generous
            shard_config.send_timeout = Some(Duration::from_mins(2));
        }

        let mut api_client = HttpClient::builder()
            .token(shard_config.token.clone())
            .token_type(client_config.token_type)
            .maybe_bot_user_agent(client_config.bot_user_agent_information)
            .build();
        if let Some(api_base_url) = client_config.api_base_url {
            api_client.api_base_url = api_base_url;
        }

        let (tx, rx) = unbounded_channel::<ClientMessage>();

        Self {
            shard_config,
            event_handlers: Vec::new(),
            context: Context {
                http_client: Arc::new(api_client),
                tx,
                cache: Arc::new(Cache::new(client_config.cache_config)),
                default_allowed_mentions: Arc::new(client_config.default_allowed_mentions),
            },
            // tx,
            rx,
            gateway_retry_wait_time_fn: client_config.gateway_retry_wait_time_fn.0,
            send_identify_presence_on_every_reconnect: client_config
                .send_initial_presence_on_every_reconnect,
            identify_presence: client_config.initial_presence,
            guild_counts_update_listeners: HashMap::new(),
            guild_members_chunk_listeners: HashMap::new(),
            latency_measurements: Vec::new(),
            initial_resume_info: client_config.resume_info,
            #[cfg(feature = "user_api")]
            subscribe_to_everything: client_config.subscribe_to_everything,
        }
    }

    #[must_use]
    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn register_event_handler(&mut self, handler: impl EventHandler + Sync + 'static) {
        self.event_handlers.push(Arc::new(handler));
    }

    /// Start the client. If an error occurs and the client needs to reconnect because of the error, an
    /// exponential backoff function is used to determine the time to wait between reconnects, which
    /// can be changed in the client config.
    ///
    /// If the client gracefully stops (initiated through `Context::gracefully_stop_client`), this will return
    /// `Ok(...)` with the resume info, if resume info is available yet (resume info may not be available if
    /// no `Ready` event has been received yet). Note that if resume info was specified in the client config,
    /// it will be returned if it has not been overwritten.
    ///
    /// # Errors
    /// Will return an error if the shard connection has been closed with an unrecoverable close code.
    /// If that happens, you should likely not try to restart the client.
    #[expect(clippy::too_many_lines, clippy::missing_panics_doc)]
    pub async fn start(&mut self) -> Result<Option<ResumeInfo>, Error> {
        tracing::debug!("Starting client.");
        #[cfg(feature = "user_api")]
        if self.subscribe_to_everything {
            let context = self.context.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::subscribe_to_everything(context).await {
                    tracing::error!("Failed to subscribe to everything: {e}");
                }
            });
        }
        let mut num_tries = 0;
        // Used for determining whether the presence should be sent on identify
        // if the client is configured to only send it once.
        let mut already_sent_presence_in_identify = false;
        let mut resume_info: Option<ResumeInfo> = self.initial_resume_info.clone();
        let mut reset_num_tries_at = Instant::now();
        'client_loop: loop {
            if reset_num_tries_at
                .checked_duration_since(Instant::now())
                .is_none()
            {
                num_tries = 0;
            }
            let (mut shard, heartbeat_interval) = match self.receive_hello().await {
                Ok(value) => value,
                Err(e) => {
                    num_tries += 1;
                    tracing::error!("(Try #{num_tries}) Error waiting for gateway `Hello`: {e}");
                    let wait_time = (self.gateway_retry_wait_time_fn)(num_tries);
                    tokio::time::sleep(wait_time).await;
                    continue;
                }
            };

            if let Some(some_resume_info) = resume_info.clone() {
                if let Err(e) = shard
                    .resume(
                        some_resume_info.session_id,
                        some_resume_info.last_sequence_number,
                    )
                    .await
                {
                    resume_info = None;
                    num_tries += 1;
                    tracing::debug!(
                        "(Try #{num_tries}) Error sending resume message to gateway, reconnecting instead: {e}"
                    );
                    let wait_time = (self.gateway_retry_wait_time_fn)(num_tries);
                    tokio::time::sleep(wait_time).await;
                    continue;
                }
            } else if let Err(e) = shard
                .identify(
                    ConnectionProperties {
                        os: String::from(std::env::consts::OS),
                        browser: String::from(crate::LIBRARY_NAME),
                        device: String::from(crate::LIBRARY_NAME),
                    },
                    if already_sent_presence_in_identify
                        && !self.send_identify_presence_on_every_reconnect
                    {
                        None
                    } else {
                        self.identify_presence.clone()
                    },
                )
                .await
            {
                num_tries += 1;
                tracing::error!(
                    "(Try #{num_tries}) Error sending identify message to gateway: {e}"
                );
                let wait_time = (self.gateway_retry_wait_time_fn)(num_tries);
                tokio::time::sleep(wait_time).await;
                continue;
            }

            reset_num_tries_at = Instant::now() + (self.gateway_retry_wait_time_fn)(num_tries);

            let mut last_heartbeat_ack_at: SystemTime = SystemTime::now();
            let no_heartbeat_ack_time_limit = heartbeat_interval * 2 + Duration::from_secs(5);
            let mut last_sequence_number = None;
            let (session_tx, mut session_rx) = unbounded_channel();
            let (shard_task_tx, mut shard_task_rx) = unbounded_channel();

            let session_tx_clone = session_tx.clone();
            tokio::spawn(async move {
                #[expect(clippy::cast_possible_truncation)]
                let first_heartbeat_wait_time = Duration::from_millis(rand::random_range(
                    0..heartbeat_interval.as_millis(),
                ) as u64);
                tokio::time::sleep(first_heartbeat_wait_time).await;
                let mut interval = tokio::time::interval(heartbeat_interval);
                // Since the first tick of the interval completes immediately, we do not need to add explicit handling for the first heartbeat.
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            if session_tx_clone.send(ClientSessionMessage::SendHeartbeat).is_err() {
                                tracing::debug!("Stopping heartbeat task.");
                                break;
                            }
                        },
                    }
                }
            });
            let session_tx_clone = session_tx.clone();
            tokio::spawn(async move {
                let maybe_error = loop {
                    tokio::select! {
                        next = shard.next_event() => {
                            let event = match next {
                                Ok(event) => event,
                                Err(e) => {
                                    tracing::error!("Error receiving next gateway event: {e:?}");
                                    // *event_receive_error_clone.lock().await = Some(e);
                                    break Some(e);
                                }
                            };
                            if session_tx_clone.send(ClientSessionMessage::GatewayEvent(Box::new(event))).is_err() {
                                break None;
                            }
                        },
                        () = session_tx_clone.closed() => {
                            break None;
                        }
                        message = shard_task_rx.recv() => {
                            let Some(message) = message else {
                                break None;
                            };
                            match message {
                                ShardTaskMessage::Send(message) => if let Err(e) = shard.send_gateway_message(message).await {
                                    tracing::error!("Error sending gateway message: {e}");
                                    break None;
                                },
                                ShardTaskMessage::SendWithResultOneshot(message, result_tx) => {
                                    let _ = result_tx.send(shard.send_gateway_message(message).await);
                                }
                                ShardTaskMessage::SendWithResultUnbounded(message, result_tx) => {
                                    let _ = result_tx.send(shard.send_gateway_message(message).await);
                                }
                            }
                        }
                    }
                };
                tracing::debug!("Stopping shard task.");
                let _ = session_tx_clone.send(ClientSessionMessage::ShardTaskEnd(
                    maybe_error.map(Into::into),
                ));
            });

            let mut queued_dispatch_events = Vec::new();
            let mut ready_received_yet = false;

            let mut session_info = SessionInfo {
                last_sequence_number: &mut last_sequence_number,
                last_heartbeat_ack_at: &mut last_heartbeat_ack_at,
                no_heartbeat_ack_time_limit,
                shard_task_tx: &shard_task_tx,
                session_tx: &session_tx,
                already_sent_presence_in_identify: &mut already_sent_presence_in_identify,
                resume_info: &mut resume_info,
                queued_dispatch_events: &mut queued_dispatch_events,
                ready_received_yet: &mut ready_received_yet,
            };

            'message_loop: loop {
                tokio::select! {
                    message = session_rx.recv() => {
                        let Some(message) = message else {
                            break 'message_loop;
                        };
                        if let ControlFlow::Break(maybe_error) = self.handle_session_message(message, &mut session_info) {
                            if let Some(error) = maybe_error {
                                match Self::check_for_unrecoverable_error(error) {
                                    Ok(e) => {
                                        tracing::error!("Error handling session message: {e}");
                                    },
                                    Err(e) => {
                                        return Err(e);
                                    }
                                }
                            }
                            continue 'client_loop;
                        }
                    }
                    message = self.rx.recv() => {
                        let Some(message) = message else {
                            panic!("There should always be at least one client message sender.");
                        };
                        if let ControlFlow::Break(gracefully_stop) = self.handle_client_message(message, &session_info) {
                            if gracefully_stop {
                                return Ok(resume_info);
                            }
                            continue 'client_loop;
                        }
                    }
                }
            }

            tracing::warn!(
                "Something happened causing all session mpsc senders to close, reconnecting."
            );
        }
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

    #[expect(clippy::result_large_err)]
    fn check_for_unrecoverable_error(error: Error) -> Result<Error, Error> {
        if let ClientErrorKind::ConnectionClosed(close_frame) = error.kind()
            && let Some(close_frame) = close_frame
            && let Some(close_code) = GatewayCloseCode::from_u16(close_frame.code.into())
            && !close_code.is_recoverable()
        {
            Err(error)
        } else {
            Ok(error)
        }
    }

    async fn receive_hello(&self) -> Result<(Shard, Duration), Error> {
        let mut shard = Shard::new(self.shard_config.clone());

        tracing::debug!("Waiting for `Hello` event from gateway.");
        let hello_event = match shard.next_event().await? {
            GatewayEventIncoming::Hello(event) => event,
            other => {
                return Err(Error::new(ClientErrorKind::UnexpectedEventReceived(
                    Box::new(other),
                )));
            }
        };

        Ok((shard, hello_event.heartbeat_interval.into()))
    }

    #[expect(clippy::too_many_lines)]
    fn handle_session_message(
        &mut self,
        message: ClientSessionMessage,
        session: &mut SessionInfo<'_>,
    ) -> ControlFlow<Option<Error>> {
        match message {
            ClientSessionMessage::SendHeartbeat => {
                let time_since_last_heartbeat_ack =
                    match SystemTime::now().duration_since(*session.last_heartbeat_ack_at) {
                        Ok(duration) => duration,
                        Err(e) => {
                            tracing::error!(
                                "Error calculating time since last heartbeat acknowledgement: {e}"
                            );
                            return ControlFlow::Break(None);
                        }
                    };
                if time_since_last_heartbeat_ack > session.no_heartbeat_ack_time_limit {
                    tracing::warn!(
                        "Received no heartbeat acknowledgement for {} seconds, reconnecting.",
                        time_since_last_heartbeat_ack.as_secs()
                    );
                    return ControlFlow::Break(None);
                }
                if session
                    .shard_task_tx
                    .send(ShardTaskMessage::Send(OutgoingGatewayMessage::Heartbeat(
                        Heartbeat {
                            last_sequence_number: *session.last_sequence_number,
                        },
                    )))
                    .is_err()
                {
                    tracing::debug!("Restarting client due to shard task being closed.");
                    return ControlFlow::Break(None);
                }
            }
            ClientSessionMessage::GatewayEvent(event) => match *event {
                GatewayEventIncoming::InvalidSession(_event) => {
                    tracing::debug!(
                        "Reconnecting because of invalid session event while waiting for `Ready`."
                    );
                    return ControlFlow::Break(None);
                }
                GatewayEventIncoming::HeartbeatAck => {
                    *session.last_heartbeat_ack_at = SystemTime::now();
                }
                GatewayEventIncoming::Hello(event) => {
                    tracing::debug!(
                        "Received second Hello message from gateway, ignoring it: {event:?}"
                    );
                }
                GatewayEventIncoming::Reconnect => {
                    tracing::debug!("Reconnecting due to gateway `Reconnect` event.");
                    return ControlFlow::Break(None);
                }
                GatewayEventIncoming::GatewayError(e) => {
                    tracing::warn!("Received gateway error event: {e:?}");
                }
                GatewayEventIncoming::Heartbeat => {
                    let _ = session.session_tx.send(ClientSessionMessage::SendHeartbeat);
                }
                GatewayEventIncoming::Dispatch(event) => {
                    match event.event {
                        DispatchEvent::Ready(ready) => {
                            *session.already_sent_presence_in_identify = true;
                            if *session.ready_received_yet {
                                tracing::warn!("Received `Ready` another time.");
                            }
                            *session.ready_received_yet = true;
                            *session.resume_info = Some(ResumeInfo {
                                session_id: ready.session_id.clone(),
                                last_sequence_number: 0,
                            });
                            self.handle_dispatch_event(DispatchEvent::Ready(ready));
                            for event in session.queued_dispatch_events.drain(..) {
                                self.handle_dispatch_event(event);
                            }
                            // Drop the old vec and set it to a vec with 0 capacity to avoid it unnecessarily holding an allocation
                            *session.queued_dispatch_events = Vec::with_capacity(0);
                        }
                        other => {
                            if session.resume_info.is_none() {
                                tracing::debug!(
                                    "Received dispatch event that is not `Ready` while waiting for `Ready`, queueing it for later."
                                );
                                session.queued_dispatch_events.push(other);
                            } else {
                                self.handle_dispatch_event(other);
                            }
                        }
                    }
                    if let Some(last_sequence_number) = session.last_sequence_number {
                        if *last_sequence_number < event.sequence_number {
                            *last_sequence_number = event.sequence_number;
                        }
                    } else {
                        *session.last_sequence_number = Some(event.sequence_number);
                    }
                    if let Some(last_sequence_number) = session.last_sequence_number
                        && let Some(resume_info) = &mut session.resume_info
                    {
                        resume_info.last_sequence_number = *last_sequence_number;
                    }
                }
            },
            ClientSessionMessage::ShardTaskEnd(maybe_error) => {
                return ControlFlow::Break(maybe_error);
            }
        }

        ControlFlow::Continue(())
    }

    /// `ControlFlow::Break(true)` if the client should gracefully stop.
    fn handle_client_message(
        &mut self,
        message: ClientMessage,
        session: &SessionInfo<'_>,
    ) -> ControlFlow<bool> {
        match message {
            ClientMessage::UpdatePresence(data, sender) => {
                if session
                    .shard_task_tx
                    .send(ShardTaskMessage::SendWithResultOneshot(
                        OutgoingGatewayMessage::PresenceUpdate(data),
                        sender,
                    ))
                    .is_err()
                {
                    tracing::debug!("Shard task exited, restarting.");
                    return ControlFlow::Break(false);
                }
            }
            ClientMessage::SendLazyRequest(data, sender) => {
                if session
                    .shard_task_tx
                    .send(ShardTaskMessage::SendWithResultUnbounded(
                        OutgoingGatewayMessage::LazyRequest(data),
                        sender,
                    ))
                    .is_err()
                {
                    tracing::debug!("Shard task exited, restarting.");
                    return ControlFlow::Break(false);
                }
            }
            ClientMessage::RequestGuildMembers(data, result_sender, tx) => {
                if let Some(tx) = tx {
                    self.guild_members_chunk_listeners
                        .insert(data.nonce.clone().unwrap(), tx);
                }
                if session
                    .shard_task_tx
                    .send(ShardTaskMessage::SendWithResultOneshot(
                        OutgoingGatewayMessage::RequestGuildMembers(data),
                        result_sender,
                    ))
                    .is_err()
                {
                    tracing::debug!("Shard task exited, restarting.");
                    return ControlFlow::Break(false);
                }
            }
            ClientMessage::LatencyMeasurement(tx) => {
                self.latency_measurements.push(tx);
                if session
                    .shard_task_tx
                    .send(ShardTaskMessage::Send(OutgoingGatewayMessage::Heartbeat(
                        Heartbeat {
                            last_sequence_number: *session.last_sequence_number,
                        },
                    )))
                    .is_err()
                {
                    tracing::debug!("Shard task exited, restarting.");
                    return ControlFlow::Break(false);
                }
            }
            ClientMessage::RequestGuildCounts(data, result_sender, tx) => {
                if let Some(tx) = tx {
                    self.guild_counts_update_listeners
                        .insert(data.nonce.clone().unwrap(), tx);
                }
                if session
                    .shard_task_tx
                    .send(ShardTaskMessage::SendWithResultOneshot(
                        OutgoingGatewayMessage::RequestGuildCounts(data),
                        result_sender,
                    ))
                    .is_err()
                {
                    tracing::debug!("Shard task exited, restarting.");
                    return ControlFlow::Break(false);
                }
            }
            ClientMessage::GracefullyStop => {
                return ControlFlow::Break(true);
            }
        }

        ControlFlow::Continue(())
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
