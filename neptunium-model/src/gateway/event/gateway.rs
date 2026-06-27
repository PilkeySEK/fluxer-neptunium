use serde::{
    Deserialize, Serialize,
    de::{self, Error, Unexpected},
};
use serde_json::{Value, json};

use crate::gateway::{
    event::{
        dispatch::DispatchEventPayload, invalid_session::InvalidSessionEvent, op_code::OpCode,
    },
    payload::{
        incoming::{GatewayError, Hello},
        outgoing::Identify,
    },
};

#[expect(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum GatewayEvent {
    Dispatch(DispatchEventPayload),
    Heartbeat,
    HeartbeatAck,
    Hello(Hello),
    InvalidSession(InvalidSessionEvent),
    Reconnect,
    GatewayError(GatewayError),
    Identify(Identify),
}

impl<'de> Deserialize<'de> for GatewayEvent {
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
        let op = OpCode::from(op as u8)
            .ok_or_else(|| de::Error::invalid_value(Unexpected::Unsigned(op), &"a valid opcode"))?;

        let d = raw["d"].clone();

        Ok(match op {
            OpCode::Dispatch => {
                Self::Dispatch(serde_json::from_value(raw).map_err(de::Error::custom)?)
            }
            OpCode::Hello => Self::Hello(serde_json::from_value(d).map_err(de::Error::custom)?),
            OpCode::Heartbeat => Self::Heartbeat,
            OpCode::HeartbeatAck => Self::HeartbeatAck,
            OpCode::InvalidSession => {
                Self::InvalidSession(serde_json::from_value(d).map_err(de::Error::custom)?)
            }
            OpCode::Reconnect => Self::Reconnect,
            OpCode::Identify => {
                Self::Identify(serde_json::from_value(d).map_err(de::Error::custom)?)
            }
            opcode => {
                return Err(D::Error::custom(format!(
                    "Not yet supported opcode: {opcode:?}"
                )));
            }
        })
    }
}

impl Serialize for GatewayEvent {
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
                    op: OpCode,
                }

                Wrapper {
                    data,
                    op: OpCode::Dispatch,
                }
                .serialize(serializer)
            }
            Self::GatewayError(data) => json!({
                "op": OpCode::GatewayError,
                "d": data,
            })
            .serialize(serializer),
            Self::Heartbeat => json!({
                "op": OpCode::Heartbeat,
                "d": serde_json::Value::Null,
            })
            .serialize(serializer),
            Self::HeartbeatAck => json!({
                "op": OpCode::HeartbeatAck,
                "d": serde_json::Value::Null,
            })
            .serialize(serializer),
            Self::Hello(data) => json!({
                "op": OpCode::Hello,
                "d": data,
            })
            .serialize(serializer),
            Self::InvalidSession(InvalidSessionEvent { resumable }) => json!({
                "op": OpCode::InvalidSession,
                "d": resumable,
            })
            .serialize(serializer),
            Self::Reconnect => json!({
                "op": OpCode::Reconnect,
                "d": serde_json::Value::Null,
            })
            .serialize(serializer),
            Self::Identify(data) => json!({
                "op": OpCode::Identify,
                "d": data,
            })
            .serialize(serializer),
        }
    }
}
