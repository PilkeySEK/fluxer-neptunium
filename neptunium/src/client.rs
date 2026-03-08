use std::time::Duration;

use fluxer_gateway::shard::{EventReceiveError, Shard, config::ShardConfig};
use fluxer_model::gateway::{
    event::gateway::GatewayEvent,
    payload::outgoing::{
        OutgoingGatewayMessage, heartbeat::Heartbeat, identify::ConnectionProperties,
    },
};
use tokio::sync::mpsc::unbounded_channel;

use crate::client::error::Error;

pub mod error;

enum ClientMessage {
    Heartbeat,
}

pub struct Client {
    shard: Shard,
}

impl Client {
    #[must_use]
    pub fn new(shard_config: ShardConfig) -> Self {
        Self {
            shard: Shard::new(shard_config),
        }
    }

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

        let (tx, mut rx) = unbounded_channel::<ClientMessage>();

        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let millis = rand::random_range(0..heartbeat_interval.as_millis() as u64);
            tokio::time::sleep(Duration::from_millis(millis)).await;
            let _ = tx_clone.send(ClientMessage::Heartbeat);
        });

        let tx_clone = tx.clone();
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

        let last_sequence_number = None;

        loop {
            tokio::select! {
                message = rx.recv() => {
                    let Some(message) = message else {
                        tracing::debug!("Channel closed, exiting.");
                        return Ok(());
                    };
                    match message {
                        ClientMessage::Heartbeat => {
                            tracing::debug!("Sending heartbeat.");
                            self.shard
                                .send_gateway_message(OutgoingGatewayMessage::Heartbeat(Heartbeat {
                                    last_sequence_number,
                                }))
                                .await?;
                        }
                    }
                },
                message = self.shard.next_event() => {
                    let message = match message {
                        Ok(message) => message,
                        Err(EventReceiveError::ParseError(e)) => {
                            tracing::warn!("Failed to parse: {}", e);
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
                    };
                    tracing::trace!("Received message: {message:?}");
                }
            }
        }
    }
}
