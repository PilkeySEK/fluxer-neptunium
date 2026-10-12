use std::collections::HashMap;

use bon::Builder;
use neptunium_model::{
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
    user::settings::{
        GuildMuteConfiguration, UserGuildSettings, UserGuildSettingsChannelOverride,
        UserNotificationSettings,
    },
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct UpdateUserGuildSettingsBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_notifications: Option<UserNotificationSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_config: Option<GuildMuteConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_push: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_everyone: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_roles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_muted_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_overrides: Option<HashMap<Id<ChannelMarker>, UserGuildSettingsChannelOverride>>,
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateUserGuildSettings {
    pub guild_id: Id<GuildMarker>,
    pub body: UpdateUserGuildSettingsBody,
}

impl Endpoint for UpdateUserGuildSettings {
    type Response = UserGuildSettings;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/users/@me/guilds/{}/settings", self.guild_id))
            .build()
    }
}
