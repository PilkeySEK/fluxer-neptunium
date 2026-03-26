use serde::{Serialize, ser::SerializeStruct};

use crate::gateway::event::op_code::OpCode;

mod heartbeat;
mod identify;
mod presence_update;
mod resume;

pub use heartbeat::*;
pub use identify::*;
pub use presence_update::*;
pub use resume::*;

#[derive(Clone, Debug)]
pub enum OutgoingGatewayMessage {
    Identify(Identify),
    Heartbeat(Heartbeat),
    PresenceUpdate(PresenceUpdateOutgoing),
    Resume(Resume),
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
        } as u8;
        s.serialize_field("op", &op)?;
        match self {
            Self::Heartbeat(d) => s.serialize_field("d", d),
            Self::Identify(d) => s.serialize_field("d", d),
            Self::PresenceUpdate(d) => s.serialize_field("d", d),
            Self::Resume(d) => s.serialize_field("d", d),
        }?;

        s.end()
    }
}
