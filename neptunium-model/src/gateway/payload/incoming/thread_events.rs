use crate::{
    channel::thread::{ThreadChannel, ThreadMember, ThreadType}, gateway::presence::{CustomStatus, PresenceStatus}, guild::member::GuildMember, id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    }, time::timestamp::{Timestamp, representations::Iso8601}
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ThreadCreate {
    #[serde(flatten)]
    pub thread: ThreadChannel,
    /// `Some(true)` when the thread was just created, `None` when the session was added to an existing thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newly_created: Option<bool>,
    /// The session's own membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<ThreadMember>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ThreadDelete {
    pub id: Id<ChannelMarker>,
    pub guild_id: Id<GuildMarker>,
    pub parent_id: Id<ChannelMarker>,
    #[serde(rename = "type")]
    pub r#type: ThreadType,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ThreadListSync {
    pub guild_id: Id<GuildMarker>,
    /// `None` for the whole guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<Id<ChannelMarker>>>,
    pub threads: Vec<ThreadChannel>,
    pub members: Vec<ThreadMember>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ThreadMemberUpdate {
    #[serde(flatten)]
    pub thread_member: ThreadMember,
    pub guild_id: Id<GuildMarker>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ThreadMembersUpdate {
    /// The thread ID.
    pub id: Id<ChannelMarker>,
    pub guild_id: Id<GuildMarker>,
    /// The number of thread members, max. `50`.
    pub member_count: usize,
    // TODO: https://web.fluxer.app/channels/1484905952721252427/1484905952721252430/1563642795653070848
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_members: Option<Vec<ThreadMember>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_member_ids: Option<Vec<Id<UserMarker>>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ThreadMemberListUpdate {
    pub guild_id: Id<GuildMarker>,
    pub thread_id: Id<ChannelMarker>,
    pub members: Vec<ThreadMemberListItem>,
}

// Absolute proper documentation (not) https://docs.fluxer.app/gateway/threads/#thread_member_list_update
// Code: https://github.com/fluxerapp/fluxer/blob/63dd214f53619b39567e9eefddfa5e463d4ca760/fluxer_app/src/features/threads/state/ThreadRoster.ts#L16
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ThreadMemberListItem {
    pub user_id: Id<UserMarker>,
    pub join_timestamp: Option<Timestamp<Iso8601>>,
    /// Undocumented (?)
    pub flags: u64,
    pub member: Option<GuildMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<ThreadMemberListItemPresence>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ThreadMemberListItemPresence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PresenceStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_status: Option<CustomStatus>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ForumUnreads {
    pub guild_id: Id<GuildMarker>,
    pub channel_id: Id<ChannelMarker>,
    pub threads: Vec<ForumUnread>,
}

// TODO: Proper enum for this?
/// Has either `count` or `missing`. `count` is the number of messages after the
/// `ack_message_id` in the request, capped at `25`.
/// `missing` is `true` when the request had no valid `ack_message_id` for that post.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ForumUnread {
    pub thread_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<bool>,
}