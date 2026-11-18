use bon::Builder;
use neptunium_model::{
    gateway::payload::incoming::UserPremiumType,
    guild::member::{GuildMember, GuildMemberProfile},
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
    user::{PartialUser, UserConnection, UserProfileData},
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
pub struct GetUserProfileParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[builder(default = false)]
    pub with_mutual_friends: bool,
    #[builder(default = false)]
    pub with_mutual_guilds: bool,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetUserProfile {
    pub user_id: Id<UserMarker>,
    pub params: GetUserProfileParams,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct MutualGuild {
    pub id: Id<GuildMarker>,
    pub nick: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct UserProfileFullResponse {
    pub user: PartialUser,
    pub user_profile: UserProfileData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_member: Option<GuildMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_member_profile: Option<GuildMemberProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<UserPremiumType>,
    // ISO8601
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<jiff::Timestamp>,
    /// Visionary ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_lifetime_seqence: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_friends: Option<Vec<PartialUser>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_guilds: Option<Vec<MutualGuild>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_accounts: Option<Vec<UserConnection>>,
}

impl Endpoint for GetUserProfile {
    type Response = UserProfileFullResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .params(serde_qs::to_string(&self.params).unwrap())
            .path(format!("/users/{}/profile", self.user_id))
            .build()
    }
}
