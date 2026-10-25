use bon::Builder;
use neptunium_model::{
    guild::member::JoinSourceType,
    id::{
        Id,
        composite::CompositeId,
        marker::{GuildMarker, RoleMarker, UserMarker},
    },
    time::timestamp::{Timestamp, representations::UnixMillis},
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Copy, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SearchGuildMembersSortBy {
    JoinedAt,
    Relevance,
}
#[derive(Serialize, Copy, Clone, Debug)]
pub enum SearchGuildMembersSortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Builder, Serialize, Clone, Debug)]
pub struct SearchGuildMembersBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub query: Option<String>,
    /// 1-100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    /// Number of members to skip for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    /// Member must have all specified roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<Id<RoleMarker>>>,
    // TODO: Is this millis or seconds?
    /// Joined at or after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at_gte: Option<Timestamp<UnixMillis>>,
    // TODO: Is this millis or seconds?
    /// Joined at or before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at_lte: Option<Timestamp<UnixMillis>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_source_type: Option<Vec<JoinSourceType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_invite_code: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,
    // TODO: Is this millis or seconds?
    /// Account created at or after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_created_at_gte: Option<Timestamp<UnixMillis>>,
    // TODO: Is this millis or seconds?
    /// Account created at or before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_created_at_lte: Option<Timestamp<UnixMillis>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SearchGuildMembersSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SearchGuildMembersSortOrder>,
}

#[derive(Builder, Clone, Debug)]
pub struct SearchGuildMembers {
    pub guild_id: Id<GuildMarker>,
    pub body: SearchGuildMembersBody,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct SearchGuildMembersSupplementalMemberDataResponse {
    /// Invite code used to join.
    pub source_invite_code: Option<String>,
    /// ID of the member who sent the invite.
    pub inviter_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_source_type: Option<JoinSourceType>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct SearchGuildMembersMemberResponse {
    pub id: CompositeId<GuildMarker, UserMarker>,
    pub guild_id: Id<GuildMarker>,
    pub user_id: Id<UserMarker>,
    pub username: String,
    // TODO: Maybe make this u16 instead
    pub discriminator: String,
    pub global_name: Option<String>,
    pub nickname: Option<String>,
    pub role_ids: Vec<Id<RoleMarker>>,
    /// Supplemental members-search-only metadata.
    pub supplemental: SearchGuildMembersSupplementalMemberDataResponse,
    pub is_bot: bool,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct SearchGuildMembersResponse {
    pub guild_id: Id<GuildMarker>,
    pub members: Vec<SearchGuildMembersMemberResponse>,
    pub page_result_count: u64,
    pub total_result_count: u64,
    /// Whether the guild members are currently being indexed.
    pub indexing: bool,
}

impl Endpoint for SearchGuildMembers {
    type Response = SearchGuildMembersResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/members-search", self.guild_id))
            .build()
    }
}
