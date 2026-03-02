use std::fmt::Display;

use fluxer_gateway::{
    client::GatewayClientError,
    model::event::{GatewayEvent, IncomingGatewayEventData, IncomingGatewayOpCode},
};

#[derive(Debug)]
pub enum NeptuniumErrorKind {
    GatewayClientError(GatewayClientError),
    UnexpectedEvent(GatewayEvent<IncomingGatewayEventData, IncomingGatewayOpCode>),
    SessionInvalidated,
}

#[derive(Debug)]
pub struct Error {
    kind: NeptuniumErrorKind,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            NeptuniumErrorKind::GatewayClientError(e) => e.fmt(f),
            NeptuniumErrorKind::UnexpectedEvent(event) => {
                f.write_fmt(format_args!("Unexpected event: {:?}", event.payload))
            }
            NeptuniumErrorKind::SessionInvalidated => f.write_str(
                "Session invalidated and the gateway reported that reconnecting is not possible",
            ),
        }
    }
}

impl Error {
    #[must_use]
    pub(crate) fn new(kind: NeptuniumErrorKind) -> Self {
        Self { kind }
    }
}

impl From<GatewayClientError> for Error {
    fn from(value: GatewayClientError) -> Self {
        Self::new(NeptuniumErrorKind::GatewayClientError(value))
    }
}
