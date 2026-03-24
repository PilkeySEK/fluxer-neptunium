use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::RoleMarker},
    misc::{HexColor32, ImageHash},
    time::timestamp::{Timestamp, representations::Iso8601},
    user::PartialUser,
};

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct GuildMemberProfileFlags: u32 {
        const AVATAR_UNSET = 1 << 0;
        const BANNER_UNSET = 1 << 1;
    }
}

impl Serialize for GuildMemberProfileFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.bits().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GuildMemberProfileFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_bits_truncate(u32::deserialize(deserializer)?))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<HexColor32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<ImageHash>,
    /// Timestamp until which the member is timed out.
    #[serde(skip_serializing_if = "Option::is_none")]
    communication_disabled_until: Option<Timestamp<Iso8601>>,
    deaf: bool,
    joined_at: Timestamp<Iso8601>,
    mute: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_flags: Option<GuildMemberProfileFlags>,
    roles: Vec<Id<RoleMarker>>,
    user: PartialUser,
}
