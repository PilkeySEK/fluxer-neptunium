use bon::Builder;
use serde::Serialize;
use serde_json::json;

use crate::{
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
    misc::ListOrSingleValue,
};

// Source: https://github.com/fluxerapp/fluxer/blob/ee1f27fe1a372b5291aead8042944afd706bf5db/fluxer_app/src/lib/GatewaySocket.tsx#L324
// Discord userdocs source: https://docs.discord.food/gateway/gateway-events#request-guild-members
#[derive(Serialize, Clone, Debug, Builder)]
pub struct RequestGuildMembers {
    #[builder(into)]
    pub guild_id: ListOrSingleValue<Id<GuildMarker>>,
    #[serde(flatten)]
    #[builder(into)]
    pub query: RequestGuildMembersQuery,
    /// 0-100, maximum number of members to send.
    /// Must be `0` when `query` is `Empty`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    /// Whether to request the presences of matching members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presences: Option<bool>,
    // TODO: Chars or bytes? Discord userdocs say that it's bytes.
    /// A nonce that the gateway will reply with, allowing you to identify
    /// which request caused the response. Maximum 32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

#[derive(Clone, Debug)]
pub enum RequestGuildMembersQuery {
    /// String that the username/nickname starts with.
    Text(String),
    MultipleIds(Vec<Id<UserMarker>>),
    SingleId(Id<UserMarker>),
    /// Equivalent to `Text(String::from(""))`.
    /// Request all members no matter their name or ID.
    Empty,
}

impl Serialize for RequestGuildMembersQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Text(s) => json!({
                "query": s,
            })
            .serialize(serializer),
            Self::MultipleIds(ids) => json!({
                "user_ids": ids,
            })
            .serialize(serializer),
            Self::SingleId(id) => json!({
                "user_ids": id,
            })
            .serialize(serializer),
            Self::Empty => json!({
                "query": "",
            })
            .serialize(serializer),
        }
    }
}

impl From<String> for RequestGuildMembersQuery {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for RequestGuildMembersQuery {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<Vec<Id<UserMarker>>> for RequestGuildMembersQuery {
    fn from(value: Vec<Id<UserMarker>>) -> Self {
        Self::MultipleIds(value)
    }
}

impl From<Id<UserMarker>> for RequestGuildMembersQuery {
    fn from(value: Id<UserMarker>) -> Self {
        Self::SingleId(value)
    }
}
