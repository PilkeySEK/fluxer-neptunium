use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{RoleMarker, UserMarker},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AllowedMentionParseType {
    Users,
    Roles,
    Everyone,
}

/// Sending this with every field as `None` resolves to an empty parse set, so no user mention,
/// role mention, `@everyone` or `@here` becomes active.
#[derive(Serialize, Deserialize, Clone, Debug, Builder, PartialEq, Eq)]
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
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied_user: Option<bool>,
}
