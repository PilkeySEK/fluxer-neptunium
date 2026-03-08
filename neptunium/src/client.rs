use std::{sync::Arc, time::Duration};

use fluxer_gateway::shard::{EventReceiveError, Shard, config::ShardConfig};
use fluxer_model::gateway::{
    event::{dispatch::DispatchEvent, gateway::GatewayEvent},
    payload::outgoing::{
        OutgoingGatewayMessage, heartbeat::Heartbeat, identify::ConnectionProperties,
    },
};
use tokio::sync::{
    //Mutex,
    mpsc::{UnboundedSender, unbounded_channel},
};

use crate::{
    client::error::{ClientErrorKind, Error},
    events::{EventHandler, context::Context},
};

pub mod api_info;
pub mod error;

enum ClientMessage {
    Heartbeat,
}

pub struct Client {
    shard: Shard,
    event_handlers: Vec<Box<dyn EventHandler>>,
    last_sequence_number: Option<u64>,
    context: Context,
}

impl Client {
    pub const DEFAULT_API_BASE_PATH: &str = "https://api.fluxer.app/v1";
    pub const USER_AGENT: &str = "Fluxer-Neptunium";

    #[must_use]
    pub fn new(shard_config: ShardConfig) -> Self {
        let token_clone = shard_config.token.clone();
        Self {
            shard: Shard::new(shard_config),
            event_handlers: Vec::new(),
            last_sequence_number: None,
            context: Context {
                api_info: Arc::new(api_info::ApiInfo {
                    token: token_clone,
                    base_path: Self::DEFAULT_API_BASE_PATH.to_owned(),
                    client: reqwest::Client::default(),
                    user_agent: format!("{}/{}", Self::USER_AGENT, crate::VERSION),
                }),
            },
        }
    }

    pub fn register_event_handler(&mut self, handler: impl EventHandler + 'static) {
        self.event_handlers
            .push(Box::new(handler) as Box<dyn EventHandler>);
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
                                    last_sequence_number: self.last_sequence_number,
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
                        Err(EventReceiveError::Closed(frame)) => {
                            let error = Error::new(error::ClientErrorKind::ConnectionClosed(frame));
                            tracing::debug!("{error}");
                            return Err(error);
                        }
                    };
                    tracing::trace!("Received message: {message:?}");
                    self.handle_event(message, &tx).await?
                }
            }
        }
    }

    async fn handle_event(
        &mut self,
        event: GatewayEvent,
        tx: &UnboundedSender<ClientMessage>,
    ) -> Result<(), self::error::Error> {
        match event {
            GatewayEvent::Heartbeat => {
                let _ = tx.send(ClientMessage::Heartbeat);
            }
            GatewayEvent::HeartbeatAck => {}
            GatewayEvent::Hello(hello) => {
                tracing::warn!("Received `Hello` event more than one time. Received: {hello:?}");
            }
            GatewayEvent::InvalidateSession { resumable } => {
                if resumable {
                    todo!(
                        "Session was invalidated and resumable is set to true, but this has not been implemented yet."
                    )
                } else {
                    return Err(Error::new(ClientErrorKind::SessionInvalidated));
                }
            }
            GatewayEvent::Reconnect => todo!("Reconnecting is not yet implemented"),
            GatewayEvent::Dispatch(payload) => {
                // Maybe check if the current sequence number is bigger than the received one because that shouldn't happen? Or something...
                self.last_sequence_number = Some(payload.sequence_number);
                return self.handle_dispatch_event(payload.event).await;
            }
        }
        Ok(())
    }

    async fn handle_dispatch_event(
        &mut self,
        event: DispatchEvent,
    ) -> Result<(), self::error::Error> {
        match event {
            DispatchEvent::Ready(data) => {
                for handler in &mut self.event_handlers {
                    handler.on_ready(self.context.clone(), &data).await;
                }
            }
            DispatchEvent::MessageCreate(data) => {
                for handler in &mut self.event_handlers {
                    handler.on_message(self.context.clone(), &data).await;
                }
            }
            DispatchEvent::GuildCreate(data) => {
                for handler in &mut self.event_handlers {
                    handler.on_guild_create(self.context.clone(), &data).await;
                }
            }
            DispatchEvent::GuildDelete(data) => {
                for handler in &mut self.event_handlers {
                    handler.on_guild_delete(self.context.clone(), &data).await;
                }
            }
            DispatchEvent::TypingStart(data) => {
                for handler in &mut self.event_handlers {
                    handler.on_typing_start(self.context.clone(), &data).await;
                }
            }
        }
        Ok(())
    }
}
