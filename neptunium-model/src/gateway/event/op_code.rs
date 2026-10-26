use serde_repr::{Deserialize_repr, Serialize_repr};

/// An opcode sent server->client.
#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IncomingOpCode {
    Dispatch = 0,
    Heartbeat = 1,
    Reconnect = 7,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,
    GatewayError = 12,
}

impl TryFrom<u8> for IncomingOpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Dispatch,
            1 => Self::Heartbeat,
            7 => Self::Reconnect,
            9 => Self::InvalidSession,
            10 => Self::Hello,
            11 => Self::HeartbeatAck,
            12 => Self::GatewayError,
            _ => return Err(()),
        })
    }
}

/// An opcode sent client->server.
#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OutgoingOpCode {
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    /// Reserved. The gateway closes a client that sends this opcode.
    VoiceServerPing = 5,
    Resume = 6,
    RequestGuildMembers = 8,
    VoiceSignal = 13,
    LazyRequest = 14,
    RequestGuildCounts = 15,
    RequestChannelMemberCounts = 16,
    RequestForumUnreads = 28,
}

impl TryFrom<u8> for OutgoingOpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Heartbeat,
            2 => Self::Identify,
            3 => Self::PresenceUpdate,
            4 => Self::VoiceStateUpdate,
            5 => Self::VoiceServerPing,
            6 => Self::Resume,
            8 => Self::RequestGuildMembers,
            13 => Self::VoiceSignal,
            14 => Self::LazyRequest,
            15 => Self::RequestGuildCounts,
            16 => Self::RequestChannelMemberCounts,
            28 => Self::RequestForumUnreads,
            _ => return Err(()),
        })
    }
}

/*
#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OpCode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,
    GatewayError = 12,
    LazyRequest = 14,
    RequestGuildCounts = 15,
}

impl OpCode {
    #[must_use]
    pub fn from(code: u8) -> Option<Self> {
        Some(match code {
            0 => Self::Dispatch,
            1 => Self::Heartbeat,
            2 => Self::Identify,
            3 => Self::PresenceUpdate,
            4 => Self::VoiceStateUpdate,
            6 => Self::Resume,
            7 => Self::Reconnect,
            8 => Self::RequestGuildMembers,
            9 => Self::InvalidSession,
            10 => Self::Hello,
            11 => Self::HeartbeatAck,
            12 => Self::GatewayError,
            14 => Self::LazyRequest,
            15 => Self::RequestGuildCounts,
            _ => return None,
        })
    }
}
*/
