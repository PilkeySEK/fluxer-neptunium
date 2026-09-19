use std::{
    env::consts,
    sync::Arc,
    time::{Duration, Instant},
};

use neptunium_model::gateway::{
    event::{dispatch::DispatchEvent, gateway::GatewayEventIncoming},
    payload::outgoing::{Identify, IdentifyProperties, OutgoingGatewayMessage, Resume},
};
use serde::{Deserialize, Serialize};
use tokio::sync::{
    OnceCell,
    mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    oneshot,
};
use tokio_util::{sync::CancellationToken, task::TaskTracker, time::FutureExt};
use zeroize::Zeroizing;

use crate::session::{
    config::SessionConfig,
    connection::{Connection, ConnectionState},
};

pub mod config;
mod error;
pub use error::*;
mod connection;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ResumeInfo {
    pub session_id: Zeroizing<String>,
    #[serde(rename = "seq")]
    pub last_sequence_number: u64,
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
}

impl Session {
    pub async fn connect(
        config: SessionConfig,
    ) -> Result<Self, tokio_tungstenite::tungstenite::Error> {
        let cancellation_token = CancellationToken::new();
        let tracker = TaskTracker::new();
        let conn = Connection::connect_and_await_hello(format!(
            "{}?{}",
            config.gateway_base_url,
            serde_qs::to_string(&config.connection_params).unwrap()
        ))
        .await?;
        let (heartbeat_task_tx, heartbeat_task_rx) = unbounded_channel();
        tracker.spawn(heartbeat_task(
            heartbeat_task_tx,
            conn.heartbeat_interval,
            cancellation_token.clone(),
        ));
        Ok(Self {
            conn,
            token: config.token,
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
        })
    }

    pub async fn run_cancellable<T: Send + Sync + 'static, Fut: Future<Output = ()>>(
        &mut self,
        mut event_handler: impl FnMut(DispatchEvent) -> Fut,
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
                event = self.next_event_with_timeout_and_heartbeats_and_identifying_or_resuming() => {
                    tracing::trace!(?event, "Received event");
                    if let Err(e) = self.handle_event(event, &mut event_handler).await {
                        break Err(e);
                    }
                }
            }
        };

        cancellation_token.cancel();
        tracker.close();
        tracker.wait().await;

        result
    }

    async fn handle_event<Fut: Future<Output = ()>>(
        &mut self,
        event: GatewayEventIncoming,
        event_handler: &mut impl FnMut(DispatchEvent) -> Fut,
    ) -> Result<(), SessionError> {
        match event {
            GatewayEventIncoming::Heartbeat => {
                self.send_heartbeat().await;
            }
            GatewayEventIncoming::HeartbeatAck => {
                self.conn.last_heartbeat_ack_at = Instant::now();
            }
            GatewayEventIncoming::Hello(event) => {
                tracing::debug!(?event, "Unexpected `Hello`");
            }
            GatewayEventIncoming::InvalidSession(event) => {
                if !event.resumable {
                    return Err(SessionError::InvalidSessionUnresumable);
                } else {
                    self.conn.reconnect_with_backoff(None).await;
                }
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
                        self.conn.state = ConnectionState::Ready;
                        self.resume_info_session_id = Some(ready.session_id.clone());
                        event_handler(DispatchEvent::Ready(ready)).await;
                    }
                    DispatchEvent::Resumed(resumed) => {
                        self.conn.state = ConnectionState::Ready;
                        event_handler(DispatchEvent::Resumed(resumed)).await;
                    }
                    event => event_handler(event).await,
                }
            }
        }
        Ok(())
    }

    /*
    async fn connection_process(&mut self) -> Result<(), SessionError> {
        let identify = OutgoingGatewayMessage::Identify(Identify {
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
            // TODO:
            shard: None,
            // TODO:
            presence: None,
            ignored_events: None,
            flags: None,
            initial_guild_id: None,
        });
        loop {
            self.conn.send_message(&identify).await;

            let next_event = self.next_event_with_timeout_and_heartbeats().await;

            match  {
                GatewayEventIncoming::Heartbeat => {
                    self.conn
                        .send_message(&OutgoingGatewayMessage::Heartbeat(Heartbeat {
                            last_sequence_number: self.last_sequence_number,
                        }))
                        .await;
                }
                GatewayEventIncoming::HeartbeatAck => {
                    self.conn.last_heartbeat_ack_at = Instant::now();
                }
                event => {
                    tracing::warn!(?event, "Unexpected event received");
                }
            }
        }
    }
    */

    async fn next_event_with_timeout_and_heartbeats_and_identifying_or_resuming(
        &mut self,
    ) -> GatewayEventIncoming {
        loop {
            self.maybe_identify_or_resume().await;
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
            let event = match maybe_event {
                Ok(event) => event,
                Err(e) => {
                    tracing::error!("Timed out waiting for heartbeat acknowledgement: {e}");
                    self.conn.reconnect_with_backoff(None).await;
                    continue;
                }
            };
            break event;
        }
    }

    async fn maybe_identify_or_resume(&mut self) {
        let message = self.create_identify_or_resume_message();
        loop {
            match self.conn.state {
                ConnectionState::Initial => {
                    if let OutgoingGatewayMessage::Resume(_) = &message {
                        self.conn.state = ConnectionState::Resuming;
                    } else {
                        self.conn.state = ConnectionState::Identifying;
                    }
                    self.last_sequence_number = None;
                    self.conn.send_message(&message).await;
                }
                ConnectionState::Ready
                | ConnectionState::Resuming
                | ConnectionState::Identifying => break,
            }
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
                // TODO:
                shard: None,
                // TODO:
                presence: None,
                ignored_events: None,
                flags: None,
                initial_guild_id: None,
            })
        }
    }

    fn respawn_heartbeat_task(&mut self) {
        let (heartbeat_task_tx, heartbeat_task_rx) = unbounded_channel();
        self.heartbeat_task_rx = heartbeat_task_rx;
        self.tracker.spawn(heartbeat_task(
            heartbeat_task_tx,
            self.conn.heartbeat_interval,
            self.cancellation_token.clone(),
        ));
    }

    async fn send_heartbeat(&mut self) {
        todo!()
    }
}

async fn heartbeat_task(
    tx: UnboundedSender<()>,
    heartbeat_interval: Duration,
    cancellation_token: CancellationToken,
) {
    let mut interval = tokio::time::interval(heartbeat_interval);
    // The first tick completes immediately
    interval.tick().await;
    loop {
        tokio::select! {
            _ = interval.tick() => {
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
