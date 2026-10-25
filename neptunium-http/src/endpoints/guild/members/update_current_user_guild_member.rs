use bon::Builder;
use neptunium_model::{
    guild::member::{GuildMember, GuildMemberProfileFlags},
    id::{Id, marker::GuildMarker},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

// TODO: Maybe allow people to set undefined and null seperately here
// TODO: Check whether setting the values to null works to reset, and check whether undefined is also an option
#[derive(Builder, Serialize, Clone, Debug)]
pub struct UpdateCurrentUserGuildMemberBody {
    #[builder(into)]
    pub nick: Option<String>,
    #[builder(into)]
    pub avatar: Option<String>,
    #[builder(into)]
    pub banner: Option<String>,
    #[builder(into)]
    pub bio: Option<String>,
    #[builder(into)]
    pub pronouns: Option<String>,
    #[builder(into)]
    pub accent_color: Option<String>,
    pub profile_flags: Option<GuildMemberProfileFlags>,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateCurrentUserGuildMember {
    pub guild_id: Id<GuildMarker>,
    pub body: UpdateCurrentUserGuildMemberBody,
}

impl Endpoint for UpdateCurrentUserGuildMember {
    type Response = GuildMember;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/guilds/{}/members/@me", self.guild_id))
            .build()
    }
}
