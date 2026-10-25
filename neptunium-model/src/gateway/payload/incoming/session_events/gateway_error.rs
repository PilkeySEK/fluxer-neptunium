use serde::Serialize;

// Source: https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/constants/src/GatewayConstants.tsx#L61
#[derive(Serialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GatewayErrorCode {
    DmInvalidChannelType,
    DmNotRecipient,
    UnknownError,
    VoiceChannelFull,
    VoiceChannelNotFound,
    VoiceConnectionNotFound,
    VoiceGuildIdMissing,
    VoiceGuildNotFound,
    VoiceInvalidChannelId,
    VoiceInvalidChannelType,
    VoiceInvalidGuildId,
    VoiceInvalidState,
    VoiceInvalidUserId,
    VoiceMemberNotFound,
    VoiceMemberTimedOut,
    VoiceMissingConnectionId,
    VoicePermissionDenied,
    VoiceTokenFailed,
    VoiceUnclaimedAccount,
    VoiceUserMismatch,
    VoiceUserNotInVoice,
}

#[derive(Serialize, Clone, Debug)]
pub struct GatewayError {
    pub code: GatewayErrorCode,
    pub message: String,
}
