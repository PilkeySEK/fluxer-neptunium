use std::collections::HashMap;

use bon::Builder;
use neptunium_model::{
    channel::{Channel, ChannelType, PermissionOverwrite, VoiceRegion},
    id::{
        Id,
        marker::{ChannelMarker, UserMarker},
    },
    time::duration::{Duration, Seconds},
};
use reqwest::Method;
use serde::Serialize;
use serde_json::{Number, Value};

use crate::{endpoints::Endpoint, request::Request};

/*
 * TODO: So, might just want to have a single struct instead of all this enum stuff, even though it
 * makes the code "less correct". Might have to choose DX over correctness, idk...
 * -> Maybe have this be as correct as possible but the neptunium crate implements nice abstractions
 * for DX or something
 */

#[derive(Serialize, Builder, Clone, Debug)]
pub struct GuildTextChannelSettingsUpdates {
    /// 1-1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// The parent category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Slowmode delay. Must be `0 <= delay <= 21600` (21600 seconds = 6 hours).
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "rate_limit_per_user"
    )]
    pub slowmode: Option<Duration<Seconds>>,
    /// The channel name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Builder, Clone, Debug)]
pub struct GuildVoiceChannelSettingsUpdates {
    /// 1-1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// The parent category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Slowmode delay. Must be `0 <= delay <= 21600` (21600 seconds = 6 hours).
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "rate_limit_per_user"
    )]
    pub slowmode: Option<Duration<Seconds>>,
    /// The channel name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The voice channel bitrate in bits per second (8000-320000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    /// Maximum users allowed in the voice channel. 0-99, where 0 means unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<VoiceRegion>,
}

#[derive(Serialize, Builder, Clone, Debug)]
pub struct GroupDmChannelSettingsUpdates {
    /// 1-1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// The parent category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Slowmode delay. Must be `0 <= delay <= 21600` (21600 seconds = 6 hours).
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "rate_limit_per_user"
    )]
    pub slowmode: Option<Duration<Seconds>>,
    /// The channel name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Base64 encoded icon image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The new owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Id<UserMarker>>,
    /// Custom nicknames for users in this group DM. Set a value in the `HashMap` to `None` to clear the custom name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nicks: Option<HashMap<Id<UserMarker>, Option<String>>>,
}

#[derive(Serialize, Builder, Clone, Debug)]
pub struct GuildCategoryChannelSettingsUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    /// The channel name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Builder, Clone, Debug)]
pub struct GuildLinkChannelSettingsUpdates {
    /// The parent category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug)]
pub enum ChannelSettingsUpdates {
    GuildText(GuildTextChannelSettingsUpdates),
    GuildVoice(GuildVoiceChannelSettingsUpdates),
    GroupDm(GroupDmChannelSettingsUpdates),
    GuildCategory(GuildCategoryChannelSettingsUpdates),
    GuildLink(GuildLinkChannelSettingsUpdates),
}

impl Serialize for ChannelSettingsUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut body = match &self {
            Self::GuildText(data) => serde_json::to_value(data).unwrap(),
            Self::GuildVoice(data) => serde_json::to_value(data).unwrap(),
            Self::GroupDm(data) => serde_json::to_value(data).unwrap(),
            Self::GuildCategory(data) => serde_json::to_value(data).unwrap(),
            Self::GuildLink(data) => serde_json::to_value(data).unwrap(),
        };
        let channel_type = match self {
            Self::GuildText(_) => ChannelType::GuildText,
            Self::GuildVoice(_) => ChannelType::GuildVoice,
            Self::GroupDm(_) => ChannelType::GroupDm,
            Self::GuildCategory(_) => ChannelType::GuildCategory,
            Self::GuildLink(_) => ChannelType::GuildLink,
        } as u16;

        body["type"] = Value::Number(Number::from_u128(u128::from(channel_type)).unwrap());

        body.serialize(serializer)
    }
}

#[derive(Builder, Clone, Debug)]
pub struct UpdateChannelSettings {
    pub channel_id: Id<ChannelMarker>,
    pub updates: ChannelSettingsUpdates,
}

impl Endpoint for UpdateChannelSettings {
    type Response = Channel;

    fn into_request(self) -> crate::request::Request {
        let req = Request::builder()
            .path(format!("/channels/{}", self.channel_id))
            .method(Method::PATCH);
        req.body(serde_json::to_string(&self.updates).unwrap())
            .build()
    }
}
