use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use neptunium_model::gateway::{
    event::gateway::GatewayEvent,
    payload::outgoing::{ConnectionProperties, Identify, OutgoingGatewayMessage, Resume},
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
use zeroize::Zeroizing;

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
    ParseError(serde_path_to_error::Error<serde_json::Error>),
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
        // serde_json::from_str(message.as_str()).map_err(EventReceiveError::ParseError)
        let result = serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(
            message.as_str(),
        ))
        .map_err(EventReceiveError::ParseError);
        if result.is_err() {
            tracing::debug!("Error while parsing: {}", message);
        }
        result
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
            shard: Some(self.config.shard_info),
            presence: None,
            ignored_events: self.config.ignored_events,
        }))
        .await
    }

    /// Close the shard connection. This also closes the underlying websocket.
    /// If no connection has been opened yet, immediately returns `Ok`.
    /// # Errors
    /// Returns an error if closing the websocket failed.
    #[expect(clippy::missing_panics_doc)]
    pub async fn close(&mut self) -> Result<(), Error> {
        if let Some(connection) = self.connection.take() {
            connection
                .rx
                .reunite(connection.tx)
                .unwrap()
                .close(None)
                .await?;
        }

        Ok(())
    }

    /// Reopens the existing connection (if there is one) and resumes.
    /// # Resuming
    /// Upon sending the `Resume` event, the gateway will replay all events after the specified
    /// `last_sequence_number`, and finally send a `Resumed` dispatch event to signal that the replaying
    /// is finished and all messages from that point onward are new.
    /// # Errors
    /// Returns an error if sending the resume message failed. **Does not** error if closing the websocket failed.
    pub async fn resume(
        &mut self,
        session_id: Zeroizing<String>,
        last_sequence_number: u64,
    ) -> Result<(), Error> {
        let _ = self.close().await;

        self.send_gateway_message(OutgoingGatewayMessage::Resume(Resume {
            token: self.config.token.clone(),
            session_id,
            seq: last_sequence_number,
        }))
        .await
    }
}
