use std::time::Duration;

use neptunium_gateway::shard::{Shard, config::ShardConfig};
use neptunium_model::gateway::{
    close_code::GatewayCloseCode,
    event::gateway::GatewayEventIncoming,
    payload::outgoing::{OutgoingGatewayMessage, PresenceUpdateOutgoing},
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::instrument;

use crate::client::error::{ClientErrorKind, Error};

pub struct SessionConfig {
    pub initial_presence: Option<PresenceUpdateOutgoing>,
    pub send_initial_presence_on_every_reconnect: bool,
    pub shard_config: ShardConfig,
}

pub struct SessionMessage {
    
}

pub struct Session<'a> {
    config: &'a SessionConfig,
    /*last_sequence_number: Option<u64>,
    last_heartbeat_ack_at: SystemTime,
    no_heartbeat_ack_time_limit: Duration,
    shard_task_tx: &'a UnboundedSender<ShardTaskMessage>,
    session_tx: &'a UnboundedSender<ClientSessionMessage>,
    already_sent_presence_in_identify: bool,
    resume_info: &'a mut Option<ResumeInfo>,
    queued_dispatch_events: &'a mut Vec<DispatchEvent>,*/
}

struct ShardConnectionError {
    /// If set to `false`, return from the session instead of reconnecting.
    pub reconnect: bool,
    e: Option<Error>,
}

impl<'a> Session<'a> {
    pub fn new(config: &'a SessionConfig) -> Self {
        Self { config }
    }

    #[instrument(skip(self, cancellation_future))]
    pub async fn run_cancellable(
        mut self,
        cancellation_future: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), Error> {
        let tracker = TaskTracker::new();
        let cancellation_token = CancellationToken::new();

        let cancellation_token_clone = cancellation_token.clone();
        tracker.spawn(async move {
            tokio::select! {
                () = cancellation_future => {
                    cancellation_token_clone.cancel();
                }
                () = cancellation_token_clone.cancelled() => {},
            }
        });

        loop {
            let connection_tracker = TaskTracker::new();
            match self
                .connection(&connection_tracker, cancellation_token.clone())
                .await
            {
                _ => todo!(),
            }
        }
    }

    async fn connection(
        &mut self,
        tracker: &TaskTracker,
        cancellation_token: CancellationToken,
    ) -> Result<(), ShardConnectionError> {
        let shard = Shard::new(self.config.shard_config.clone());
        let (shard_send_tx, mut incoming) = unbounded_channel();
        let (outgoing, shard_recv_rx) = unbounded_channel();
        let shard_task_result = tracker.spawn(shard_task(
            shard,
            shard_send_tx,
            shard_recv_rx,
            cancellation_token.clone(),
        ));

        loop {
            tokio::select! {
                () = cancellation_token.cancelled() => {
                   break;
                }
                msg = incoming.recv() => {
                    let Some(msg) = msg else {
                        match tokio::time::timeout(Duration::from_secs(1), shard_task_result).await {
                            Err(_) => {
                                tracing::error!("Timeout waiting for shard task to be finished");
                                return Err(ShardConnectionError { reconnect: true, e: None });
                            },
                            Ok(Err(join_error)) => {
                                tracing::error!("Join error waiting for shard task to be finished: {join_error}");
                               return Err(ShardConnectionError { reconnect: true, e: None });
                            }
                            Ok(Ok(Err(e))) => {
                                return Err(shard_error_to_connection_error(e));
                            }
                            Ok(Ok(Ok(()))) => {}
                        }
                        break;
                    };
                    self.handle_incoming_event(msg).await?;
                }
            }
        }

        Ok(())
    }

    async fn handle_incoming_event(
        &mut self,
        event: GatewayEventIncoming,
    ) -> Result<(), ShardConnectionError> {
        todo!()
    }
}

async fn shard_task(
    shard: Shard,
    tx: UnboundedSender<GatewayEventIncoming>,
    rx: UnboundedReceiver<OutgoingGatewayMessage>,
    cancellation_token: CancellationToken,
) -> Result<(), Error> {
    Ok(())
}

fn shard_error_to_connection_error(e: Error) -> ShardConnectionError {
    if let Error {
        kind: ClientErrorKind::ConnectionClosed(Some(CloseFrame { code, .. })),
    } = e
        && let Some(code) = GatewayCloseCode::from_u16(code.into())
        && !code.is_recoverable()
    {
        ShardConnectionError {
            reconnect: false,
            e: Some(e),
        }
    } else {
        ShardConnectionError {
            reconnect: true,
            e: Some(e),
        }
    }
}
