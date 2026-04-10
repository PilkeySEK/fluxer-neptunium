use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    id::{Id, marker::RoleMarker},
    misc::{HexColor32, ImageHash},
    time::timestamp::{Timestamp, representations::Iso8601},
    user::PartialUser,
};

/// How a member joined a guild.
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum JoinSourceType {
    /// Member created the guild.
    Creator = 0,
    /// Member joined via an instant invite.
    InstantInvite = 1,
    /// Member joined via the vanity url.
    VanityUrl = 2,
    /// Member was added via a bot invite.
    BotInvite = 3,
    /// Member was force-added by a platform administrator.
    AdminForceAdd = 4,
}

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
    pub avatar: Option<ImageHash>,
    /// Timestamp until which the member is timed out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<Timestamp<Iso8601>>,
    // May be not present when the payload is of type `GuildMemberData` and not `GuildMemberResponse`.
    #[serde(default = "boolean_false")]
    pub deaf: bool,
    pub joined_at: Timestamp<Iso8601>,
    // May be not present when the payload is of type `GuildMemberData` and not `GuildMemberResponse`.
    #[serde(default = "boolean_false")]
    pub mute: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_flags: Option<GuildMemberProfileFlags>,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: PartialUser,
}

// I hate serde what even is this
fn boolean_false() -> bool {
    false
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberProfile {
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub banner: Option<ImageHash>,
    pub accent_color: Option<HexColor32>,
}
