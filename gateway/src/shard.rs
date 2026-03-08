use fluxer_model::gateway::{
    event::gateway::GatewayEvent,
    payload::outgoing::{
        OutgoingGatewayMessage,
        identify::{ConnectionProperties, Identify},
    },
};
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, Message, protocol::CloseFrame},
};

#[derive(Debug)]
struct ShardConnection {
    tx: SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    rx: SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
}

pub use tungstenite::Error;

use crate::shard::config::ShardConfig;
pub mod config;

#[derive(Debug)]
pub struct Shard {
    config: ShardConfig,
    connection: Option<ShardConnection>,
}

#[derive(Debug)]
pub enum EventReceiveError {
    TungsteniteError(Error),
    ParseError(serde_json::Error),
    UnsupportedMessageEncoding,
    Closed(Option<CloseFrame>),
}

impl Shard {
    #[must_use]
    pub fn new(config: ShardConfig) -> Self {
        Self {
            config,
            connection: None,
        }
    }

    /// Get an immutable reference to the `ShardConfig`.
    #[must_use]
    pub fn config(&self) -> &ShardConfig {
        &self.config
    }

    /// Either returns the existing connection or establishes a new connection and then returns it (and sets `connection` to `Some(...)`).
    /// After calling this function it is guaranteed that the `connection` field is `Some`.
    async fn get_connection(&mut self) -> Result<&mut ShardConnection, Error> {
        match &mut self.connection {
            Some(conn) => Ok(conn),
            none => {
                let (stream, _) = connect_async(&self.config.gateway_url).await?;
                let (tx, rx) = stream.split();
                Ok(none.insert(ShardConnection { tx, rx }))
            }
        }
    }

    /// # Errors
    /// Returns an error if receiving the message fails.
    pub async fn next_message(&mut self) -> Result<Message, Error> {
        let connection = self.get_connection().await?;
        let message = connection
            .rx
            .next()
            .await
            .unwrap_or_else(|| Err(Error::ConnectionClosed))?;
        tracing::trace!("Received: {message:?}");
        Ok(message)
    }

    /// # Errors
    /// Returns an error if receiving the event fails.
    pub async fn next_event(&mut self) -> Result<GatewayEvent, EventReceiveError> {
        let message = self
            .next_message()
            .await
            .map_err(EventReceiveError::TungsteniteError)?;
        if let Message::Close(frame) = message {
            return Err(EventReceiveError::Closed(frame));
        }
        let Message::Text(message) = message else {
            return Err(EventReceiveError::UnsupportedMessageEncoding);
        };
        serde_json::from_str(message.as_str()).map_err(EventReceiveError::ParseError)
    }

    /// # Errors
    /// Returns an error if sending the message fails.
    pub async fn send_message(&mut self, message: Message) -> Result<(), Error> {
        let connection = self.get_connection().await?;
        connection.tx.send(message).await
    }

    /// # Errors
    /// Returns an error if sending the message fails.
    #[expect(clippy::missing_panics_doc)]
    pub async fn send_gateway_message(
        &mut self,
        message: OutgoingGatewayMessage,
    ) -> Result<(), Error> {
        self.send_message(Message::Text(
            serde_json::to_string(&message).unwrap().into(),
        ))
        .await
    }

    /// Helper function for sending the identify message to the gateway. This should be used after a `Hello` message is received.
    /// # Errors
    /// Returns an error if sending the message fails.
    pub async fn identify(
        &mut self,
        connection_properties: ConnectionProperties,
    ) -> Result<(), Error> {
        self.send_gateway_message(OutgoingGatewayMessage::Identify(Identify {
            token: self.config.token.clone(),
            properties: connection_properties,
            compress: false,
            large_threshold: None,
            shard: Some(self.config.shard_info),
            presence: None,
            intents: self.config.intents,
        }))
        .await
    }
}
