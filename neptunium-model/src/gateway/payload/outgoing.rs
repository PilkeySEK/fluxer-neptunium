use serde::{Deserialize, Serialize, ser::SerializeStruct};

use crate::gateway::event::op_code::OutgoingOpCode;

mod heartbeat;
mod identify;
mod lazy_request;
mod presence_update;
mod request_channel_member_counts;
mod request_forum_unreads;
mod request_guild_counts;
mod request_guild_members;
mod resume;
mod voice_signal;
mod voice_state_update;

pub use heartbeat::*;
pub use identify::*;
pub use lazy_request::*;
pub use presence_update::*;
pub use request_channel_member_counts::*;
pub use request_forum_unreads::*;
pub use request_guild_counts::*;
pub use request_guild_members::*;
pub use resume::*;
pub use voice_signal::*;
pub use voice_state_update::*;

#[derive(Clone, Debug)]
#[expect(clippy::large_enum_variant)]
pub enum OutgoingGatewayMessage {
    Heartbeat(Heartbeat),
    Identify(Identify),
    PresenceUpdate(PresenceUpdateOutgoing),
    VoiceStateUpdate(VoiceStateUpdateOutgoing),
    Resume(Resume),
    RequestGuildMembers(RequestGuildMembers),
    VoiceSignal(VoiceSignalOutgoing),
    LazyRequest(LazyRequest),
    RequestGuildCounts(RequestGuildCounts),
    RequestChannelMemberCounts(RequestChannelMemberCounts),
    RequestForumUnreads(RequestForumUnreads),
}

impl Serialize for OutgoingGatewayMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("OutgoingGatewayMessage", 2)?;

        let op = match self {
            Self::Identify(_) => OutgoingOpCode::Identify,
            Self::Heartbeat(_) => OutgoingOpCode::Heartbeat,
            Self::PresenceUpdate(_) => OutgoingOpCode::PresenceUpdate,
            Self::Resume(_) => OutgoingOpCode::Resume,
            Self::LazyRequest(_) => OutgoingOpCode::LazyRequest,
            Self::RequestGuildMembers(_) => OutgoingOpCode::RequestGuildMembers,
            Self::RequestGuildCounts(_) => OutgoingOpCode::RequestGuildCounts,
            Self::VoiceStateUpdate(_) => OutgoingOpCode::VoiceStateUpdate,
            Self::RequestForumUnreads(_) => OutgoingOpCode::RequestForumUnreads,
            Self::RequestChannelMemberCounts(_) => OutgoingOpCode::RequestChannelMemberCounts,
            Self::VoiceSignal(_) => OutgoingOpCode::VoiceSignal,
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
            Self::VoiceStateUpdate(d) => s.serialize_field("d", d),
            Self::RequestForumUnreads(d) => s.serialize_field("d", d),
            Self::RequestChannelMemberCounts(d) => s.serialize_field("d", d),
            Self::VoiceSignal(d) => s.serialize_field("d", d),
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
        let Ok(op_code) = OutgoingOpCode::try_from(schema.op) else {
            return Err(serde::de::Error::custom("unknown opcode"));
        };
        Ok(match op_code {
            OutgoingOpCode::Identify => {
                Self::Identify(serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?)
            }
            OutgoingOpCode::Heartbeat => {
                Self::Heartbeat(serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?)
            }
            OutgoingOpCode::PresenceUpdate => Self::PresenceUpdate(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OutgoingOpCode::Resume => {
                Self::Resume(serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?)
            }
            OutgoingOpCode::LazyRequest => Self::LazyRequest(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OutgoingOpCode::RequestGuildMembers => Self::RequestGuildMembers(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OutgoingOpCode::RequestGuildCounts => Self::RequestGuildCounts(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OutgoingOpCode::VoiceStateUpdate => Self::VoiceStateUpdate(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OutgoingOpCode::RequestForumUnreads => Self::RequestForumUnreads(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OutgoingOpCode::RequestChannelMemberCounts => Self::RequestChannelMemberCounts(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OutgoingOpCode::VoiceSignal => Self::VoiceSignal(
                serde_json::from_value(schema.d).map_err(serde::de::Error::custom)?,
            ),
            OutgoingOpCode::VoiceServerPing => {
                todo!("voice server ping is supposed to never be sent by clients")
            }
        })
    }
}
