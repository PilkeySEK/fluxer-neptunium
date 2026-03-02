use crate::{
    client::{GatewayClientError, GatewayClientErrorType},
    model::event::{
        GatewayEventPayload, IncomingGatewayOpCode,
        dispatch::{
            channel::{MessageCreateDispatchData, TypingStartDispatchData},
            guild::{GuildCreateDispatchData, GuildDeleteDispatchData},
            session::{ReadyDispatchData, SessionsReplaceDispatchData},
        },
    },
};

/// Sent based on channel type (guild channels use guild scope, DMs use presence scope)
pub mod channel;
/// Sent to all users in a guild who have permission to receive it
pub mod guild;
/// Sent to all sessions of the current user
pub mod presence;
/// Sent only to the current session
pub mod session;

#[derive(Debug, Clone)]
pub enum DispatchEvent {
    Ready(Box<ReadyDispatchData>),
    GuildDelete(GuildDeleteDispatchData),
    GuildCreate(Box<GuildCreateDispatchData>),
    MessageCreate(Box<MessageCreateDispatchData>),
    TypingStart(TypingStartDispatchData),
    SessionsReplace(SessionsReplaceDispatchData),
}

impl TryFrom<GatewayEventPayload<IncomingGatewayOpCode>> for DispatchEvent {
    type Error = GatewayClientError;
    fn try_from(value: GatewayEventPayload<IncomingGatewayOpCode>) -> Result<Self, Self::Error> {
        let Some(event_name) = value.t else {
            return Err(GatewayClientError::new(
                crate::client::GatewayClientErrorType::NoEventNameFieldInPayload,
            ));
        };

        Ok(match event_name.as_str() {
            "READY" => {
                let Some(d) = value.d else {
                    return Err(GatewayClientError::new(
                        crate::client::GatewayClientErrorType::NoDataFieldInPayload,
                    ));
                };
                Self::Ready(Box::new(serde_json::from_value(d).map_err(|e| {
                    GatewayClientError::new(GatewayClientErrorType::DeserializeError(e))
                })?))
            }
            "GUILD_DELETE" => {
                let Some(d) = value.d else {
                    return Err(GatewayClientError::new(
                        crate::client::GatewayClientErrorType::NoDataFieldInPayload,
                    ));
                };
                Self::GuildDelete(serde_json::from_value(d).map_err(|e| {
                    GatewayClientError::new(GatewayClientErrorType::DeserializeError(e))
                })?)
            }
            "GUILD_CREATE" => {
                let Some(d) = value.d else {
                    return Err(GatewayClientError::new(
                        crate::client::GatewayClientErrorType::NoDataFieldInPayload,
                    ));
                };
                Self::GuildCreate(Box::new(serde_json::from_value(d).map_err(|e| {
                    GatewayClientError::new(GatewayClientErrorType::DeserializeError(e))
                })?))
            }
            "MESSAGE_CREATE" => {
                let Some(d) = value.d else {
                    return Err(GatewayClientError::new(
                        crate::client::GatewayClientErrorType::NoDataFieldInPayload,
                    ));
                };
                Self::MessageCreate(Box::new(serde_json::from_value(d).map_err(|e| {
                    GatewayClientError::new(GatewayClientErrorType::DeserializeError(e))
                })?))
            }
            "TYPING_START" => {
                let Some(d) = value.d else {
                    return Err(GatewayClientError::new(
                        crate::client::GatewayClientErrorType::NoDataFieldInPayload,
                    ));
                };
                Self::TypingStart(serde_json::from_value(d).map_err(|e| {
                    GatewayClientError::new(GatewayClientErrorType::DeserializeError(e))
                })?)
            }
            "SESSIONS_REPLACE" => {
                let Some(d) = value.d else {
                    return Err(GatewayClientError::new(
                        crate::client::GatewayClientErrorType::NoDataFieldInPayload,
                    ));
                };
                Self::SessionsReplace(serde_json::from_value(d).map_err(|e| {
                    GatewayClientError::new(GatewayClientErrorType::DeserializeError(e))
                })?)
            }
            _ => {
                return Err(GatewayClientError::new(
                    GatewayClientErrorType::UnknownEvent(event_name),
                ));
            }
        })
    }
}
