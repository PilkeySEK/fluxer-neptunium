use serde::{Serialize, ser::SerializeStruct};

use crate::gateway::event::op_code::OpCode;

mod heartbeat;
mod identify;
mod lazy_request;
mod presence_update;
mod request_guild_members;
mod resume;

pub use heartbeat::*;
pub use identify::*;
pub use lazy_request::*;
pub use presence_update::*;
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
        } as u8;
        s.serialize_field("op", &op)?;
        match self {
            Self::Heartbeat(d) => s.serialize_field("d", d),
            Self::Identify(d) => s.serialize_field("d", d),
            Self::PresenceUpdate(d) => s.serialize_field("d", d),
            Self::Resume(d) => s.serialize_field("d", d),
            Self::LazyRequest(d) => s.serialize_field("d", d),
            Self::RequestGuildMembers(d) => s.serialize_field("d", d),
        }?;

        s.end()
    }
}
