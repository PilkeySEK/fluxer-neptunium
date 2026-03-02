use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

use crate::model::{
    object::{
        channel::ChannelResponse,
        gateway::CustomStatusResponse,
        user::{
            FavoriteMemeResponse, UserGuildSettingsResponse, UserPartialResponse,
            UserPrivateResponse, UserSettingsResponse,
        },
    },
    snowflake::Snowflake,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildReadyResponse {
    pub id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "time::serde::iso8601::option"
    )]
    pub joined_at: Option<OffsetDateTime>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PresenceResponse {
    pub user: UserPartialResponse,
    pub status: String,
    pub mobile: bool,
    pub afk: bool,
    pub custom_status: Option<CustomStatusResponse>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ReadStateResponse {
    pub id: Snowflake,
    pub mention_count: i32,
    pub last_message_id: Option<Snowflake>,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyDispatchData {
    pub version: i32,
    pub session_id: String,
    pub user: UserPrivateResponse,
    pub guilds: Vec<GuildReadyResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_channels: Option<Vec<ChannelResponse>>,
    /// This is `Vec<Value>` because the docs say it's type is `object[]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserPartialResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presences: Option<Vec<PresenceResponse>>,
    /// This is `Vec<Value>` because the docs say it's type is `object[]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<UserSettingsResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_guild_settings: Option<Vec<UserGuildSettingsResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_states: Option<Vec<ReadStateResponse>>,
    /// This is `Value` because the docs say it's type is `object`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Value>,
    /// Two-letter country code based on IP geolocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Array of pinned DM channel IDs
    ///
    /// **NOTE:** This was specified as "string[]" in the documentation, but
    /// appears to be a `Snowflake` since channel IDs are `Snowflake`s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_dms: Option<Vec<Snowflake>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite_memes: Option<Vec<FavoriteMemeResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_session_id_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionsReplaceDispatchData(
    /// TODO: Look at payloads to find out what is in here (documentation says object[])
    Vec<Value>,
);
