use std::collections::HashMap;

use bon::Builder;
use neptunium_model::{
    id::{Id, marker::ChannelMarker},
    user::settings::{
        GuildMuteConfiguration, UserGuildSettings, UserGuildSettingsChannelOverride,
        UserNotificationSettings,
    },
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct UpdateDmNotificationSettings {
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
    /// While this is technically accepted by the API, it doesn't make much sense to set it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_roles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_muted_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_overrides: Option<HashMap<Id<ChannelMarker>, UserGuildSettingsChannelOverride>>,
}

impl Endpoint for UpdateDmNotificationSettings {
    type Response = UserGuildSettings;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/guilds/@me/settings".to_owned())
            .build()
    }
}
