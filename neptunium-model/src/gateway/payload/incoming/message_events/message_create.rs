use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    channel::{ChannelType, message::Message},
    guild::member::GuildMemberProfileFlags,
    id::{
        Id,
        marker::{GuildMarker, RoleMarker, UserMarker},
    },
    misc::HexColor,
    time::timestamp::{Timestamp, representations::Iso8601},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageCreate {
    #[serde(flatten)]
    pub message: Message,
    pub channel_type: ChannelType,
    /// Group direct message nicknames, when the group has any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nicks: Option<HashMap<Id<UserMarker>, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_here: Option<bool>,
    /// Guild the channel belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<MessageCreateGuildMember>,
}

impl std::ops::Deref for MessageCreate {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl std::ops::DerefMut for MessageCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.message
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageCreateGuildMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<HexColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
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
}

fn boolean_false() -> bool {
    false
}
