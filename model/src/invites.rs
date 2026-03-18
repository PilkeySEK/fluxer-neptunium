use serde::{Deserialize, Serialize};

use crate::{
    channel::ChannelPartial,
    guild::GuildPartial,
    id::{
        Id,
        marker::{PackMarker, UserMarker},
    },
    time::{
        duration::{Duration, representation::Seconds},
        timestamp::{Timestamp, representations::Iso8601},
    },
    user::UserPartial,
};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PackType {
    Emoji,
    Sticker,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackInfo {
    pub id: Id<PackMarker>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub r#type: PackType,
    pub creator_id: Id<UserMarker>,
    pub created_at: Timestamp<Iso8601>,
    pub updated_at: Timestamp<Iso8601>,
    pub creator: UserPartial,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PackInviteMetadata {
    pub created_at: Timestamp<Iso8601>,
    pub uses: u64,
    pub max_uses: u64,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GroupDmInviteMetadata {
    pub created_at: Timestamp<Iso8601>,
    pub uses: u64,
    pub max_uses: u64,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GuildInviteMetadata {
    pub created_at: Timestamp<Iso8601>,
    pub uses: u64,
    pub max_uses: u64,
    /// Duration before the invite expires.
    pub max_age: Duration<Seconds>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildInvite {
    pub code: String,
    pub guild: GuildPartial,
    pub channel: ChannelPartial,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<UserPartial>,
    /// The approximate total member count of the guild.
    pub member_count: u64,
    /// The approximate online member count of the guild.
    pub presence_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp<Iso8601>>,
    #[serde(rename = "temporary")]
    pub grants_temporary_membership: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupDmInvite {
    pub code: String,
    pub channel: ChannelPartial,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<UserPartial>,
    /// The approximate total member count of the group DM.
    pub member_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp<Iso8601>>,
    #[serde(rename = "temporary")]
    pub grants_temporary_membership: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackInvite {
    pub code: String,
    pub pack: PackInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<UserPartial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp<Iso8601>>,
    #[serde(rename = "temporary")]
    pub grants_temporary_access: bool,
}

#[derive(Clone, Debug)]
pub enum Invite {
    Guild(GuildInvite),
    GroupDm(GroupDmInvite),
    EmojiPack(PackInvite),
    StickerPack(PackInvite),
}
#[derive(Clone, Debug)]
pub enum InviteWithMetadata {
    Guild(GuildInvite, GuildInviteMetadata),
    GroupDm(GroupDmInvite, GroupDmInviteMetadata),
    EmojiPack(PackInvite, PackInviteMetadata),
    StickerPack(PackInvite, PackInviteMetadata),
}

impl<'de> Deserialize<'de> for Invite {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        let mut raw = serde_json::Value::deserialize(deserializer)?;

        #[expect(clippy::cast_possible_truncation)]
        let type_tag = raw
            .get("type")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| D::Error::missing_field("type"))? as u8;

        raw.as_object_mut().unwrap().remove("type");

        match type_tag {
            0 => GuildInvite::deserialize(raw)
                .map(Self::Guild)
                .map_err(D::Error::custom),
            1 => GroupDmInvite::deserialize(raw)
                .map(Self::GroupDm)
                .map_err(D::Error::custom),
            2 => PackInvite::deserialize(raw)
                .map(Self::EmojiPack)
                .map_err(D::Error::custom),
            3 => PackInvite::deserialize(raw)
                .map(Self::StickerPack)
                .map_err(D::Error::custom),
            n => Err(D::Error::custom(format!("unknown invite type: {n}"))),
        }
    }
}

impl<'de> Deserialize<'de> for InviteWithMetadata {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        let mut raw = serde_json::Value::deserialize(deserializer)?;

        #[expect(clippy::cast_possible_truncation)]
        let type_tag = raw
            .get("type")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| D::Error::missing_field("type"))? as u8;

        raw.as_object_mut().unwrap().remove("type");

        Ok(match type_tag {
            0 => Self::Guild(
                GuildInvite::deserialize(raw.clone()).map_err(Error::custom)?,
                GuildInviteMetadata::deserialize(raw).map_err(Error::custom)?,
            ),
            1 => Self::GroupDm(
                GroupDmInvite::deserialize(raw.clone()).map_err(Error::custom)?,
                GroupDmInviteMetadata::deserialize(raw).map_err(Error::custom)?,
            ),
            2 => Self::EmojiPack(
                PackInvite::deserialize(raw.clone()).map_err(Error::custom)?,
                PackInviteMetadata::deserialize(raw).map_err(Error::custom)?,
            ),
            3 => Self::StickerPack(
                PackInvite::deserialize(raw.clone()).map_err(Error::custom)?,
                PackInviteMetadata::deserialize(raw).map_err(Error::custom)?,
            ),
            n => return Err(D::Error::custom(format!("unknown invite type: {n}"))),
        })
    }
}
