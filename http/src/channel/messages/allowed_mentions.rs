use fluxer_model::id::{
    Id,
    marker::{RoleMarker, UserMarker},
};
use serde::Serialize;

#[derive(Serialize, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AllowedMentionParseType {
    Users,
    Roles,
    Everyone,
}

/// [Source](https://github.com/fluxerapp/fluxer/blob/03813bbe17db008452f0f1be3090a7d2970a5447/packages/schema/src/domains/message/SharedMessageSchemas.tsx#L34)
#[derive(Serialize, Clone, Debug)]
pub struct AllowedMentions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<Vec<AllowedMentionParseType>>,
    /// Maximum 100 users to mention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<Id<UserMarker>>>,
    /// Maximum 100 roles to mention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Id<RoleMarker>>>,
    /// Whether to mention the author of the replied message.
    pub replied_user: bool,
}
