use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::model::{object::message::MessageResponse, snowflake::Snowflake};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCreateDispatchData {
    #[serde(flatten)]
    pub message_response: MessageResponse,
    // TODO: find out what this means
    pub channel_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMember {/* TODO */}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingStartDispatchData {
    channel_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    guild_id: Option<Snowflake>,
    user_id: Snowflake,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    timestamp: OffsetDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<GuildMember>,
}
