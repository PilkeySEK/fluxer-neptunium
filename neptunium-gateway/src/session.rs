use std::{
    env::consts,
    sync::Arc,
    time::{Duration, Instant},
};

use futures_util::{SinkExt, TryStreamExt};
use neptunium_model::gateway::{
    event::gateway::GatewayEventIncoming,
    payload::outgoing::{Heartbeat, Identify, IdentifyProperties, OutgoingGatewayMessage},
};
use serde::{Deserialize, Serialize};
use tokio::{
    net::TcpStream,
    sync::{
        OnceCell,
        mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
        oneshot,
    },
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, Utf8Bytes, protocol::CloseFrame},
};
use tokio_util::{
    sync::{CancellationToken, DropGuard},
    task::TaskTracker,
    time::FutureExt,
};
use tracing::instrument;
use zeroize::Zeroizing;

use crate::session::{
    config::{GatewayConnectionParams, SessionConfig},
    connection::Connection,
};

pub mod config;
mod error;
pub use error::*;
mod connection;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ResumeInfo {
    pub session_id: String,
    #[serde(rename = "seq")]
    pub last_sequence_number: u64,
}

pub struct Session {
    conn: Connection,
    token: Zeroizing<String>,
    // state: SessionState,
    gateway_base_url: String,
    connection_params: GatewayConnectionParams,
    resume_info: Option<ResumeInfo>,
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
            gateway_base_url: config.gateway_base_url,
            connection_params: config.connection_params,
            resume_info: config.resume_info,
            last_sequence_number: None,
            heartbeat_task_rx,
            // _cancellation_token_drop_guard: cancellation_token.drop_guard(),
            cancellation_token,
            tracker,
        })
    }

    pub async fn run_cancellable<T: Send + Sync + 'static>(
        &mut self,
        event_handler: impl FnMut(GatewayEventIncoming, &mut Self),
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
                            break Ok((self.resume_info.take(), value));
                        },
                        Err(e) => {
                            panic!("cancel value channel error: {e}");
                        }
                    }
                }
            }
        };

        cancellation_token.cancel();
        tracker.close();
        tracker.wait().await;

        result
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

    async fn next_event_with_timeout_and_heartbeats(&mut self) -> (GatewayEventIncoming, bool) {
        let mut reconnected = false;
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
            let event = match maybe_event {
                Ok((event, set_reconnected)) => {
                    if set_reconnected {
                        reconnected = true;
                    }
                    event
                }
                Err(e) => {
                    tracing::error!("Timed out waiting for heartbeat acknowledgement: {e}");
                    self.conn.reconnect_with_backoff(None).await;
                    continue;
                }
            };
            break (event, reconnected);
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
