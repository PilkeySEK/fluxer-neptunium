use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    id::{Id, marker::UserMarker},
    misc::{HexColor, serde_bitflags},
    user::flags::PublicUserFlags,
};

#[cfg(feature = "user_api")]
pub mod auth;
#[cfg(feature = "user_api")]
pub mod data_harvest;
pub mod flags;
#[cfg(feature = "user_api")]
pub mod gifts;
pub mod read_state;
pub mod relationship;
#[cfg(feature = "user_api")]
pub mod saved_messages;
pub mod settings;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PartialUser {
    pub avatar: Option<String>,
    pub avatar_color: Option<HexColor>,
    #[serde(default)]
    pub bot: bool,
    // TODO: Maybe store as u16 instead
    pub discriminator: String,
    pub flags: PublicUserFlags,
    pub global_name: Option<String>,
    pub id: Id<UserMarker>,
    #[serde(default)]
    pub system: bool,
    /// Note that this is not unique (because Fluxer has discriminators).
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_flags: Option<MentionReplyPreference>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct UserProfileData {
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub banner: Option<String>,
    pub accent_color: Option<HexColor>,
    pub banner_color: Option<HexColor>,
}

#[derive(Deserialize, Copy, Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserExternalAccountConnectionType {
    #[serde(rename = "bsky")]
    Bluesky,
    Domain,
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct UserExternalAccountConnectionVisibilityFlags: u32 {
        const EVERYONE = 1 << 0;
        const FRIENDS = 1 << 1;
        const MUTUAL_GUILDS = 1 << 2;
    }
}

serde_bitflags!(UserExternalAccountConnectionVisibilityFlags, u32);

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct UserExternalAccountConnection {
    // TODO: Is this a snowflake?
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: UserExternalAccountConnectionType,
    /// The display name of the connection (handle or domain).
    pub name: String,
    /// Whether the connection has been verified.
    pub verified: bool,
    pub visibility_flags: UserExternalAccountConnectionVisibilityFlags,
    pub sort_order: u32,
}

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MentionReplyPreference {
    NoPreference = 0,
    PreferMention = 1,
    PreferNoMention = 2,
}
