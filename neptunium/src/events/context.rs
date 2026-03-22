use std::sync::Arc;

use neptunium_http::{
    client::HttpClient,
    endpoints::gateway::get_gateway_information::{GatewayInformation, GetGatewayInformation},
};
use neptunium_model::{
    channel::Channel,
    gateway::payload::outgoing::presence_update::PresenceUpdateOutgoing,
    id::{Id, marker::ChannelMarker},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    client::{ClientMessage, error::Error},
    exts::ChannelExt,
};

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) tx: UnboundedSender<ClientMessage>,
}

impl Context {
    #[must_use]
    pub fn get_http_client(&self) -> &Arc<HttpClient> {
        &self.http_client
    }

    /// Update the presence by sending a gateway request. Due to
    /// how the crate is structured currently, this does not block.
    pub fn update_presence(&self, data: PresenceUpdateOutgoing) {
        // ignoring potential error caused by the channel being closed
        // TODO: Maybe not ignore it
        let _ = self.tx.send(ClientMessage::UpdatePresence(data));
    }

    /// Fetch a channel from the API.
    /// # Errors
    /// Returns an error if there was a network error, the API did not return OK,
    /// or the API returned unexpected data that could not be parsed.
    pub async fn fetch_channel(&self, channel_id: Id<ChannelMarker>) -> Result<Channel, Error> {
        channel_id.fetch(self).await
    }

    /// Get the gateway information from the API.
    /// # Errors
    /// Returns an error if there was a network error, the API did not return OK,
    /// or the API returned unexpected data that could not be parsed.
    pub async fn get_gateway_information(&self) -> Result<GatewayInformation, Error> {
        Ok(self.http_client.execute(GetGatewayInformation).await?)
    }
}
