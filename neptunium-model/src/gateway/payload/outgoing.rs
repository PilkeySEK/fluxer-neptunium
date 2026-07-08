use serde::{Deserialize, Serialize, ser::SerializeStruct};

use crate::gateway::event::op_code::OpCode;

mod heartbeat;
mod identify;
mod lazy_request;
mod presence_update;
mod request_guild_counts;
mod request_guild_members;
mod resume;

pub use heartbeat::*;
pub use identify::*;
pub use lazy_request::*;
pub use presence_update::*;
pub use request_guild_counts::*;
pub use request_guild_members::*;
pub use resume::*;

#[derive(Clone, Debug)]
pub enum OutgoingGatewayMessage {
    Identify(Identify),
    Heartbeat(Heartbeat),
    PresenceUpdate(PresenceUpdateOutgoing),
    Resume(Resume),
    LazyRequest(LazyRequest),
    RequestGuildMembers(RequestGuildMembers),
    RequestGuildCounts(RequestGuildCounts),
}

impl Serialize for OutgoingGatewayMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("OutgoingGatewayMessage", 2)?;

        let op = match self {
            Self::Identify(_) => OpCode::Identify,
            Self::Heartbeat(_) => OpCode::Heartbeat,
            Self::PresenceUpdate(_) => OpCode::PresenceUpdate,
            Self::Resume(_) => OpCode::Resume,
            Self::LazyRequest(_) => OpCode::LazyRequest,
            Self::RequestGuildMembers(_) => OpCode::RequestGuildMembers,
            Self::RequestGuildCounts(_) => OpCode::RequestGuildCounts,
        } as u8;
        s.serialize_field("op", &op)?;
        match self {
            Self::Heartbeat(d) => s.serialize_field("d", d),
            Self::Identify(d) => s.serialize_field("d", d),
            Self::PresenceUpdate(d) => s.serialize_field("d", d),
            Self::Resume(d) => s.serialize_field("d", d),
            Self::LazyRequest(d) => s.serialize_field("d", d),
            Self::RequestGuildMembers(d) => s.serialize_field("d", d),
            Self::RequestGuildCounts(d) => s.serialize_field("d", d),
        }?;

        s.end()
    }
}

impl<'de> Deserialize<'de> for OutgoingGatewayMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Schema {
            op: u8,
            d: serde_json::Value,
        }

        let schema = Schema::deserialize(deserializer)?;
        let Some(op_code) = OpCode::from(schema.op) else {
            return Err(serde::de::Error::custom("unknown opcode"));
        };
        Ok(match op_code {
            OpCode::Identify => {
                Self::Identify(serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?)
            }
            OpCode::Heartbeat => {
                Self::Heartbeat(serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?)
            }
            OpCode::PresenceUpdate => Self::PresenceUpdate(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OpCode::Resume => {
                Self::Resume(serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?)
            }
            OpCode::LazyRequest => Self::LazyRequest(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OpCode::RequestGuildMembers => Self::RequestGuildMembers(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OpCode::RequestGuildCounts => Self::RequestGuildCounts(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            _ => todo!(),
        })
    }
}
