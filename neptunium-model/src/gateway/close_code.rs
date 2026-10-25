/// When the gateway closes a connection, it sends a close code indicating why it
/// closed the connection. Some close codes are recoverable (the client should reconnect),
/// while others are not. Determine whether a close code is recoverable using [`is_recoverable`](Self::is_recoverable).
#[expect(clippy::doc_paragraphs_missing_punctuation)]
pub enum GatewayCloseCode {
    /// Unknown error occurred.
    ///
    ///  **Should reconnect:** Yes
    UnknownError = 4000,
    /// Sent an invalid gateway opcode.
    ///
    ///  **Should reconnect:** Yes
    UnknownOpcode = 4001,
    /// Sent an invalid payload.
    ///
    ///  **Should reconnect:** Yes
    DecodeError = 4002,
    /// Sent a payload before identifying.
    ///
    ///  **Should reconnect:** Yes
    NotAuthenticated = 4003,
    /// Account token is invalid.
    ///
    ///  **Should reconnect:** No
    AuthenticationFailed = 4004,
    /// Sent more than one identify payload.
    ///
    ///  **Should reconnect:** Yes
    AlreadyAuthenticated = 4005,
    /// Sent an invalid sequence when resuming.
    ///
    ///  **Should reconnect:** Yes
    InvalidSequence = 4007,
    /// Sending payloads too quickly.
    ///
    ///  **Should reconnect:** Yes
    RateLimited = 4008,
    /// Session timed out; reconnect and start a new one.
    ///
    ///  **Should reconnect:** Yes
    SessionTimeout = 4009,
    /// Sent an invalid shard when identifying.
    ///
    ///  **Should reconnect:** No
    InvalidShard = 4010,
    /// Session would have handled too many guilds; sharding is required.
    ///
    ///  **Should reconnect:** No
    ShardingRequired = 4011,
    /// Sent an invalid gateway version.
    ///
    ///  **Should reconnect:** No
    InvalidApiVersion = 4012,
}

impl GatewayCloseCode {
    #[must_use]
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::UnknownError
            | Self::UnknownOpcode
            | Self::DecodeError
            | Self::NotAuthenticated
            | Self::AlreadyAuthenticated
            | Self::InvalidSequence
            | Self::RateLimited
            | Self::SessionTimeout => true,
            Self::AuthenticationFailed
            | Self::InvalidShard
            | Self::ShardingRequired
            | Self::InvalidApiVersion => false,
        }
    }

    #[must_use]
    pub fn from_u16(value: u16) -> Option<Self> {
        Some(match value {
            4000 => Self::UnknownError,
            4001 => Self::UnknownOpcode,
            4002 => Self::DecodeError,
            4003 => Self::NotAuthenticated,
            4004 => Self::AuthenticationFailed,
            4005 => Self::AlreadyAuthenticated,
            4007 => Self::InvalidSequence,
            4008 => Self::RateLimited,
            4009 => Self::SessionTimeout,
            4010 => Self::InvalidShard,
            4011 => Self::ShardingRequired,
            4012 => Self::InvalidApiVersion,
            _ => return None,
        })
    }
}
