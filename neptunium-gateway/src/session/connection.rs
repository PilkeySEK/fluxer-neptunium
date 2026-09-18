use std::time::{Duration, Instant};

use neptunium_model::gateway::{
    event::gateway::GatewayEventIncoming,
    payload::{
        incoming::Hello,
        outgoing::{Heartbeat, OutgoingGatewayMessage},
    },
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, client::IntoClientRequest, protocol::CloseFrame},
};
use tracing::instrument;

use crate::session::connection::internal::{next_event, send_message};

type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct Connection {
    stream: Stream,
    request: tokio_tungstenite::tungstenite::http::Request<()>,
    pub last_heartbeat_ack_at: Instant,
    pub heartbeat_interval: Duration,
    pub state: ConnectionState,
}

pub enum ConnectionState {
    Initial,
    Resuming,
    Identified,
    Ready,
}

impl Connection {
    pub async fn connect_and_await_hello(
        request: impl IntoClientRequest,
    ) -> Result<Self, tokio_tungstenite::tungstenite::Error> {
        let request = request.into_client_request()?;

        let (mut stream, _response) = connect_async(request.clone()).await?;

        let heartbeat_interval = loop {
            match next_event(&mut stream, &request).await.0 {
                GatewayEventIncoming::Hello(Hello { heartbeat_interval }) => {
                    break heartbeat_interval.into();
                }
                GatewayEventIncoming::Heartbeat => {
                    // Respond to heartbeat requests just in case
                    send_message(
                        &mut stream,
                        &request,
                        &OutgoingGatewayMessage::Heartbeat(Heartbeat {
                            last_sequence_number: None,
                        }),
                    )
                    .await;
                }
                event => {
                    tracing::warn!(
                        ?event,
                        "Discarding event because it was received before `Hello`"
                    );
                }
            }
        };

        Ok(Self {
            stream,
            request,
            last_heartbeat_ack_at: Instant::now(),
            heartbeat_interval,
            state: ConnectionState::Initial,
        })
    }

    /// Wait for the next message from the gateway, reconnecting
    /// if the client has been disconnected.
    ///
    /// # Errors
    /// This method does not return an error because it will always try reconnecting
    /// to recover from errors.
    pub async fn next_message(&mut self) -> (Message, bool) {
        internal::next_message(&mut self.stream, &self.request).await
    }

    /// Wait for the next gateway event to be received.
    ///
    /// # Errors
    /// Will always retry if receiving the event fails, and reconnect if the
    /// received event could not be deserialized.
    pub async fn next_event(&mut self) -> (GatewayEventIncoming, bool) {
        internal::next_event(&mut self.stream, &self.request).await
    }

    /// Reconnect, waiting an increasingly longer time between reconnects if they fail.
    /// This never returns an error as it will try to reconnect again if
    /// one occurs.
    pub async fn reconnect_with_backoff(&mut self, close_frame: Option<CloseFrame>) {
        internal::reconnect_with_backoff(&mut self.stream, &self.request, close_frame).await;
    }

    /// Close the existing connection and start a new one.
    pub async fn reconnect(
        &mut self,
        close_frame: Option<CloseFrame>,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        internal::reconnect(&mut self.stream, self.request.clone(), close_frame).await
    }

    pub async fn send_message(&mut self, event: &OutgoingGatewayMessage) -> bool {
        internal::send_message(&mut self.stream, &self.request, event).await
    }
}

mod internal {
    use std::time::Duration;

    use futures_util::{SinkExt, TryStreamExt};
    use neptunium_model::gateway::{
        event::gateway::GatewayEventIncoming, payload::outgoing::OutgoingGatewayMessage,
    };
    use tokio_tungstenite::{
        connect_async,
        tungstenite::{Message, http::Request, protocol::CloseFrame},
    };

    use crate::session::connection::Stream;

    pub async fn reconnect(
        stream: &mut Stream,
        request: Request<()>,
        close_frame: Option<CloseFrame>,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        tracing::debug!("Reconnecting...");
        if let Err(e) = stream.close(close_frame).await {
            tracing::warn!("Error closing existing connection while reconnecting: {e}");
        }
        let (new_stream, _response) = connect_async(request).await?;
        *stream = new_stream;
        Ok(())
    }

    pub async fn reconnect_with_backoff(
        stream: &mut Stream,
        request: &Request<()>,
        close_frame: Option<CloseFrame>,
    ) {
        fn wait_time(num_tries: usize) -> Duration {
            if num_tries < 3 {
                Duration::from_secs(3)
            } else {
                Duration::from_secs(num_tries as u64 * 5).min(Duration::from_mins(2))
            }
        }

        let mut num_tries = 0;
        loop {
            if let Err(e) = reconnect(stream, request.clone(), close_frame.clone()).await {
                num_tries += 1;
                let wait_time = wait_time(num_tries);
                tracing::error!(
                    "Error reconnecting (try #{num_tries}), waiting {}s before reconnecting: {e}",
                    wait_time.as_secs()
                );
                tokio::time::sleep(wait_time).await;
            } else {
                break;
            }
        }
    }

    pub async fn next_message(stream: &mut Stream, request: &Request<()>) -> (Message, bool) {
        let mut reconnected = false;
        let message = loop {
            match stream.try_next().await {
                Ok(Some(message)) => break message,
                Ok(None) => {
                    reconnect_with_backoff(stream, request, None).await;
                    reconnected = true;
                },
                Err(e) => {
                    tracing::error!("{e}");
                    reconnect_with_backoff(stream, request, None).await;
                    reconnected = true;
                }
            }
        };
        (message, reconnected)
    }

    pub async fn next_event(stream: &mut Stream, request: &Request<()>) -> (GatewayEventIncoming, bool) {
        let mut reconnected = false;
        let event = loop {
            let (message, set_reconnected) = next_message(stream, request).await;
            if set_reconnected {
                reconnected = true;
            }
            match message {
                Message::Text(bytes) => match serde_json::from_str(bytes.as_str()) {
                    Ok(event) => break event,
                    Err(e) => {
                        tracing::error!(
                            message_text = bytes.as_str(),
                            "Failed to deserialize incoming event, reconnecting: {e}"
                        );
                        reconnect_with_backoff(stream, request, None).await;
                        reconnected = true;
                    }
                },
                Message::Close(frame) => {
                    tracing::debug!(close_frame = ?frame, "Gateway closed connection");
                }
                message => {
                    tracing::trace!(?message, "Message is not text, skipping it");
                }
            }
        };
        (event, reconnected)
    }

    pub async fn send_message(
        stream: &mut Stream,
        request: &Request<()>,
        event: &OutgoingGatewayMessage,
    ) -> bool {
        let mut reconnected = false;
        loop {
            if let Err(e) = stream
                .send(Message::Text(serde_json::to_string(event).unwrap().into()))
                .await
            {
                tracing::error!("Failed to send message, reconnecting and then retrying: {e}");
                reconnect_with_backoff(stream, request, None).await;
                reconnected = true;
            } else {
                break;
            }
        }
        reconnected
    }
}
