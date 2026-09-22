use std::{
    env::consts,
    sync::Arc,
    time::{Duration, Instant},
};

use neptunium_model::{
    gateway::{
        event::{
            dispatch::DispatchEvent, gateway::GatewayEventIncoming,
            invalid_session::InvalidSessionEvent,
        },
        payload::outgoing::{
            Heartbeat, Identify, IdentifyProperties, InitialPresence, OutgoingGatewayMessage,
            Resume,
        },
        shard::ShardInfo,
    },
    serde_bool,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{
    Mutex, Notify, OnceCell,
    mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    oneshot,
};
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_util::{sync::CancellationToken, task::TaskTracker, time::FutureExt};
use zeroize::Zeroizing;

use crate::session::{config::SessionConfig, connection::Connection};

pub mod config;
mod error;
pub use error::*;
mod connection;
mod handle;
pub use handle::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ResumeInfo {
    pub session_id: Zeroizing<String>,
    #[serde(rename = "seq")]
    pub last_sequence_number: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectError {
    #[error("network error: {0}")]
    Tungstenite(tokio_tungstenite::tungstenite::Error),
    #[error("disconnected with unrecoverable close code: {0:?}")]
    ClosedUnrecoverable(CloseFrame),
}

enum SessionMessage {
    SendMessage(OutgoingGatewayMessage, Arc<Notify>),
}

pub struct Session {
    conn: Connection,
    token: Zeroizing<String>,
    // state: SessionState,
    // gateway_base_url: String,
    // connection_params: GatewayConnectionParams,
    resume_info_session_id: Option<Zeroizing<String>>,
    last_sequence_number: Option<u64>,
    heartbeat_task_rx: UnboundedReceiver<()>,
    cancellation_token: CancellationToken,
    // _cancellation_token_drop_guard: DropGuard,
    tracker: TaskTracker,
    // TODO: Could refactor `Session` a little bit to avoid having to do this Arc<Mutex<T>> thing,
    // but right now it's not that important tbh
    rx: Arc<tokio::sync::Mutex<UnboundedReceiver<SessionMessage>>>,
    tx: UnboundedSender<SessionMessage>,
    // identify_or_resume: OutgoingGatewayMessage,
    send_initial_presence_on_every_reconnect: bool,
    initial_presence: Option<InitialPresence>,
    shard: Option<ShardInfo>,
    ignored_events: Option<Vec<String>>,
}

impl Session {
    pub async fn connect(config: SessionConfig) -> Result<Self, ConnectError> {
        let cancellation_token = CancellationToken::new();
        let tracker = TaskTracker::new();
        let conn = Connection::connect_and_await_hello(format!(
            "{}?{}",
            config.gateway_base_url,
            serde_qs::to_string(&config.connection_params).unwrap(),
        ))
        .await?;
        let (heartbeat_task_tx, heartbeat_task_rx) = unbounded_channel();
        tracker.spawn(heartbeat_task(
            heartbeat_task_tx,
            conn.heartbeat_interval,
            cancellation_token.clone(),
        ));
        let (tx, rx) = unbounded_channel();
        let mut this = Self {
            conn,
            token: config.token.clone(),
            // state: SessionState::default(),
            // gateway_base_url: config.gateway_base_url,
            // connection_params: config.connection_params,
            last_sequence_number: config
                .resume_info
                .as_ref()
                .map(|info| info.last_sequence_number),
            resume_info_session_id: config.resume_info.map(|info| info.session_id),
            heartbeat_task_rx,
            // _cancellation_token_drop_guard: cancellation_token.drop_guard(),
            cancellation_token,
            tracker,
            tx,
            rx: Arc::new(Mutex::new(rx)),
            send_initial_presence_on_every_reconnect: config
                .send_initial_presence_on_every_reconnect,
            initial_presence: config.initial_presence,
            shard: config.shard,
            ignored_events: config.ignored_events,
        };
        let identify_or_resume = this.create_identify_or_resume_message();
        this.conn.send_message(&identify_or_resume).await;
        Ok(this)
    }

    pub fn handle(&self) -> SessionHandle {
        SessionHandle {
            tx: self.tx.clone(),
        }
    }

    pub async fn run_cancellable<T: Send + Sync + 'static>(
        &mut self,
        mut event_handler: impl FnMut(DispatchEvent),
        cancel: impl Future<Output = T> + Send + 'static,
    ) -> Result<(Option<ResumeInfo>, T), SessionError> {
        let cancellation_token = CancellationToken::new();
        let tracker = TaskTracker::new();
        let heartbeat_interval = Arc::new(OnceCell::new());
        let (heartbeat_tx, mut heartbeat_rx) = unbounded_channel();
        {
            let heartbeat_interval = Arc::clone(&heartbeat_interval);
            let cancellation_token = cancellation_token.clone();
            tracker.spawn(async move {
                loop {
                    let wait_time = if let Some(duration) = heartbeat_interval.get() {
                        *duration
                    } else {
                        Duration::from_secs(40)
                    };
                    tokio::select! {
                        () = cancellation_token.cancelled() => {
                            break;
                        },
                        _ = tokio::time::sleep(wait_time) => {}
                    }
                    if let Err(_) = heartbeat_tx.send(()) {
                        break;
                    }
                }
            });
        }
        let (cancel_value_tx, mut cancel_value_rx) = oneshot::channel();
        {
            let cancellation_token = cancellation_token.clone();
            tracker.spawn(async move {
                tokio::select! {
                    value = cancel => {
                        let _ = cancel_value_tx.send(value);
                    }
                    () = cancellation_token.cancelled() => {},
                }
            });
        }

        let result = loop {
            let rx = Arc::clone(&self.rx);
            let mut rx = rx.lock().await;
            tokio::select! {
                msg = heartbeat_rx.recv() => {
                    let Some(()) = msg else {
                        panic!("heartbeat_tx was dropped");
                    };
                    self.send_heartbeat().await;
                }
                msg = &mut cancel_value_rx => {
                    match msg {
                        Ok(value) => {
                            break Ok((
                                if let Some(session_id) = self.resume_info_session_id.take()
                                    && let Some(last_sequence_number) = self.last_sequence_number
                                {
                                    Some(ResumeInfo {
                                        session_id,
                                        last_sequence_number,
                                    })
                                } else {
                                    None
                                },
                                value
                            ));
                        },
                        Err(e) => {
                            panic!("cancel value channel error: {e}");
                        }
                    }
                }
                // TODO: Technically cancel-unsafe
                event_result = self.next_event_with_timeout_and_heartbeats_and_identifying_or_resuming() => {
                    let event = event_result.map_err(SessionError::ClosedUnrecoverable)?;
                    tracing::trace!(?event, "Received event");
                    if let Err(e) = self.handle_event(event, &mut event_handler).await {
                        break Err(e);
                    }
                }
                msg = rx.recv() => {
                    let msg = msg.expect("channel should be open");
                    self.handle_session_message(msg).await;
                }
            }
        };

        cancellation_token.cancel();
        tracker.close();
        tracker.wait().await;

        result
    }

    async fn handle_session_message(&mut self, msg: SessionMessage) {
        match msg {
            SessionMessage::SendMessage(message, finished) => {
                self.conn.send_message(&message).await;
                finished.notify_one();
            }
        }
    }

    async fn handle_event(
        &mut self,
        event: GatewayEventIncoming,
        event_handler: &mut impl FnMut(DispatchEvent),
    ) -> Result<(), SessionError> {
        match event {
            GatewayEventIncoming::Heartbeat => {
                self.send_heartbeat().await;
            }
            GatewayEventIncoming::HeartbeatAck => {
                self.conn.last_heartbeat_ack_at = Instant::now();
            }
            GatewayEventIncoming::Hello(event) => {
                self.conn.heartbeat_interval = event.heartbeat_interval.into();
                self.respawn_heartbeat_task();
                let identify_or_resume = self.create_identify_or_resume_message();
                self.conn.send_message(&identify_or_resume).await;
            }
            GatewayEventIncoming::InvalidSession(InvalidSessionEvent {
                resumable: serde_bool::False,
            }) => {
                self.resume_info_session_id = None;
                self.conn.reconnect_with_backoff(None).await;
            }
            GatewayEventIncoming::GatewayError(event) => {
                tracing::warn!(?event, "Gateway error event received");
                self.conn.reconnect_with_backoff(None).await;
            }
            GatewayEventIncoming::Reconnect => {
                self.conn.reconnect_with_backoff(None).await;
            }
            GatewayEventIncoming::Dispatch(payload) => {
                if let Some(last_sequence_number) = self.last_sequence_number
                    && last_sequence_number > payload.sequence_number
                {
                    tracing::warn!("Stored sequence number is larger than received one");
                }
                self.last_sequence_number = Some(payload.sequence_number);
                match payload.event {
                    DispatchEvent::Ready(ready) => {
                        self.resume_info_session_id = Some(ready.session_id.clone());
                        event_handler(DispatchEvent::Ready(ready));
                    }
                    DispatchEvent::Resumed(resumed) => {
                        event_handler(DispatchEvent::Resumed(resumed));
                    }
                    event => event_handler(event),
                }
            }
        }
        Ok(())
    }

    async fn next_event_with_timeout_and_heartbeats_and_identifying_or_resuming(
        &mut self,
    ) -> Result<GatewayEventIncoming, CloseFrame> {
        loop {
            let heartbeat_ack_timeout_at =
                self.conn.last_heartbeat_ack_at + (self.conn.heartbeat_interval * 2);
            let maybe_event;
            loop {
                tokio::select! {
                    v = self.conn.next_event().timeout_at(heartbeat_ack_timeout_at.into()) => {
                        maybe_event = v;
                        break;
                    }
                    heartbeat = self.heartbeat_task_rx.recv() => {
                        if heartbeat.is_some() {
                            self.send_heartbeat().await;
                        } else {
                            tracing::warn!("Heartbeat task stopped, respawning it");
                            self.respawn_heartbeat_task();
                        }
                    }
                }
            }
            let event_result = match maybe_event {
                Ok(event) => event,
                Err(e) => {
                    tracing::error!("Timed out waiting for heartbeat acknowledgement: {e}");
                    self.conn.reconnect_with_backoff(None).await;
                    continue;
                }
            };
            break event_result;
        }
    }

    /// Either `Resume` or `Identify`.
    fn create_identify_or_resume_message(&mut self) -> OutgoingGatewayMessage {
        if let Some(session_id) = self.resume_info_session_id.take()
            && let Some(seq) = self.last_sequence_number
        {
            OutgoingGatewayMessage::Resume(Resume {
                token: self.token.clone(),
                session_id,
                seq,
            })
        } else {
            OutgoingGatewayMessage::Identify(Identify {
                token: self.token.clone(),
                properties: IdentifyProperties {
                    os: consts::OS.to_owned(),
                    browser: env!("CARGO_CRATE_NAME").to_owned(),
                    device: "desktop".to_owned(),
                    // TODO:
                    e2ee_capable: None,
                    mobile: None,
                    latitude: None,
                    longitude: None,
                },
                shard: self.shard,
                presence: if self.send_initial_presence_on_every_reconnect {
                    self.initial_presence.clone()
                } else {
                    self.initial_presence.take()
                },
                ignored_events: self.ignored_events.clone(),
                flags: None,
                initial_guild_id: None,
            })
        }
    }

    fn respawn_heartbeat_task(&mut self) {
        tracing::debug!("Respawning heartbeat task");
        let (heartbeat_task_tx, heartbeat_task_rx) = unbounded_channel();
        self.heartbeat_task_rx = heartbeat_task_rx;
        self.tracker.spawn(heartbeat_task(
            heartbeat_task_tx,
            self.conn.heartbeat_interval,
            self.cancellation_token.clone(),
        ));
    }

    async fn send_heartbeat(&mut self) {
        self.conn
            .send_message(&OutgoingGatewayMessage::Heartbeat(Heartbeat {
                last_sequence_number: self.last_sequence_number,
            }))
            .await;
    }
}

#[tracing::instrument(skip(tx, cancellation_token))]
async fn heartbeat_task(
    tx: UnboundedSender<()>,
    heartbeat_interval: Duration,
    cancellation_token: CancellationToken,
) {
    let mut interval = tokio::time::interval(heartbeat_interval);
    // The first tick completes immediately
    interval.tick().await;
    // TODO: Is there something like waiting random(0..heartbeat_interval) time on the first heartbeat documented?
    loop {
        tokio::select! {
            _ = interval.tick() => {
                tracing::trace!("Sending heartbeat over channel");
                if tx.send(()).is_err() {
                    tracing::debug!("Stopping heartbeat task because receiver has been dropped");
                    break;
                }
            }
            () = cancellation_token.cancelled() => {
                tracing::debug!("Stopping heartbeat task because it has been cancelled");
                break;
            }
        }
    }
}
