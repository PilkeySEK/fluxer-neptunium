use std::sync::Arc;

use neptunium_model::gateway::payload::outgoing::OutgoingGatewayMessage;
use tokio::sync::{Notify, mpsc::UnboundedSender};

use crate::session::SessionMessage;

#[derive(Clone)]
pub struct SessionHandle {
    pub(super) tx: UnboundedSender<SessionMessage>,
}

impl SessionHandle {
    /// Send a message to the gateway.
    /// Returns `false` if the `Session` that this `Handle` is referencing has been dropped.
    pub async fn send_message(&self, message: OutgoingGatewayMessage) -> bool {
        let notify = Arc::new(Notify::new());
        if self
            .tx
            .send(SessionMessage::SendMessage(message, Arc::clone(&notify)))
            .is_err()
        {
            return false;
        }
        notify.notified().await;
        true
    }

    /// Returns `true` if the `Session` that this `Handle` is referencing has been dropped.
    pub fn is_closed(&self) -> bool {
        self.tx.is_closed()
    }
}
