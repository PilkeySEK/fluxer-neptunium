use serde::{
    Deserialize, Serialize,
    de::{self, Error},
};
use serde_json::{Value, json};

use crate::gateway::{
    event::{
        dispatch::DispatchEventPayload, invalid_session::InvalidSessionEvent,
        op_code::IncomingOpCode,
    },
    payload::incoming::{GatewayError, Hello},
};

#[expect(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum GatewayEventIncoming {
    Dispatch(DispatchEventPayload),
    Heartbeat,
    HeartbeatAck,
    Hello(Hello),
    InvalidSession(InvalidSessionEvent),
    Reconnect,
    GatewayError(GatewayError),
    /// The fluxer docs say that the client should ignore a server
    /// opcode that is not documented.
    Ignored(u64),
}

impl<'de> Deserialize<'de> for GatewayEventIncoming {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: Optimize this by using a visitor or something
        let raw = Value::deserialize(deserializer)?;
        let op = raw
            .get("op")
            .and_then(Value::as_u64)
            .ok_or_else(|| de::Error::missing_field("op"))?;
        #[expect(clippy::cast_possible_truncation)]
        let Ok(op) = IncomingOpCode::try_from(op as u8) else {
            return Ok(Self::Ignored(op));
        };

        let d = raw["d"].clone();

        Ok(match op {
            IncomingOpCode::Dispatch => {
                Self::Dispatch(serde_json::from_value(raw).map_err(de::Error::custom)?)
            }
            IncomingOpCode::Hello => {
                Self::Hello(serde_json::from_value(d).map_err(de::Error::custom)?)
            }
            IncomingOpCode::Heartbeat => Self::Heartbeat,
            IncomingOpCode::HeartbeatAck => Self::HeartbeatAck,
            IncomingOpCode::InvalidSession => {
                Self::InvalidSession(serde_json::from_value(d).map_err(de::Error::custom)?)
            }
            IncomingOpCode::Reconnect => Self::Reconnect,
            opcode @ IncomingOpCode::GatewayError => {
                return Err(D::Error::custom(format!(
                    "Not yet supported opcode: {opcode:?}"
                )));
            }
        })
    }
}

impl Serialize for GatewayEventIncoming {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Dispatch(data) => {
                #[derive(Serialize)]
                struct Wrapper<'a> {
                    #[serde(flatten)]
                    data: &'a DispatchEventPayload,
                    op: IncomingOpCode,
                }

                Wrapper {
                    data,
                    op: IncomingOpCode::Dispatch,
                }
                .serialize(serializer)
            }
            Self::GatewayError(data) => json!({
                "op": IncomingOpCode::GatewayError,
                "d": data,
            })
            .serialize(serializer),
            Self::Heartbeat => json!({
                "op": IncomingOpCode::Heartbeat,
                "d": serde_json::Value::Null,
            })
            .serialize(serializer),
            Self::HeartbeatAck => json!({
                "op": IncomingOpCode::HeartbeatAck,
                "d": serde_json::Value::Null,
            })
            .serialize(serializer),
            Self::Hello(data) => json!({
                "op": IncomingOpCode::Hello,
                "d": data,
            })
            .serialize(serializer),
            Self::InvalidSession(InvalidSessionEvent { resumable }) => json!({
                "op": IncomingOpCode::InvalidSession,
                "d": resumable,
            })
            .serialize(serializer),
            Self::Reconnect => json!({
                "op": IncomingOpCode::Reconnect,
                "d": serde_json::Value::Null,
            })
            .serialize(serializer),
            Self::Ignored(_) => Err(serde::ser::Error::custom(
                "cannot serializer Ignored variant",
            )),
        }
    }
}
