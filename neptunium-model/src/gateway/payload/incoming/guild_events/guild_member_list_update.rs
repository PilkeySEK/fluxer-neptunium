use serde::Deserialize;

use crate::{
    gateway::presence::CustomStatus,
    guild::member::GuildMember,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, RoleMarker},
    },
};

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberListUpdate {
    pub guild_id: Id<GuildMarker>,
    /// The channel for which the member list is updated.
    pub id: Id<ChannelMarker>,
    /// Same as `id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    pub member_count: usize,
    pub online_count: usize,
    pub groups: Vec<MemberListGroup>,
    pub ops: Vec<MemberListOperation>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MemberListGroup {
    /// This is usually the role ID, but it may also be
    /// "online" or "offline".
    pub id: MemberListGroupId,
    pub count: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum MemberListGroupId {
    Id(Id<RoleMarker>),
    Online,
    Offline,
}

impl<'de> Deserialize<'de> for MemberListGroupId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        match string.as_str() {
            "online" => Ok(Self::Online),
            "offline" => Ok(Self::Offline),
            other => Ok(Self::Id(Id::<RoleMarker>::try_from(other).map_err(
                |_| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(other),
                        &"a valid snowflake",
                    )
                },
            )?)),
        }
    }
}

#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum MemberListOperationType {
    Sync,
    Insert,
    Update,
    Delete,
    Invalidate,
}

// TODO: Find out which of these are sent for which ops
#[derive(Deserialize, Clone, Debug)]
pub struct MemberListOperation {
    pub op: MemberListOperationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<(usize, usize)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MemberListItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<MemberListItem>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MemberListItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<MemberListGroup>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MemberListGuildMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<MemberListPresence>,
    #[serde(flatten)]
    pub member: GuildMember,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MemberListPresence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_status: Option<CustomStatus>,
}
