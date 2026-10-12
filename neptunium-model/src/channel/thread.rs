use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    guild::member::GuildMember,
    id::{
        Id,
        marker::{ChannelMarker, ForumTagMarker, GuildMarker, MessageMarker, UserMarker},
    },
    misc::serde_bitflags,
    time::{
        duration::{Duration, Seconds},
        timestamp::{Timestamp, representations::Iso8601},
    },
    user::settings::GuildMuteConfiguration,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ThreadChannel {
    /// A thread started from a message has the ID of that message.
    /// The first message of a forum post has the ID of the post.
    pub id: Id<ChannelMarker>,
    #[serde(rename = "type")]
    pub r#type: ThreadType,
    pub guild_id: Id<GuildMarker>,
    /// The ID of the channel that the text belongs to.
    pub parent_id: Id<ChannelMarker>,
    /// The ID of the user or webhook that created the thread.
    pub owner_id: Id<UserMarker>,
    pub name: String,
    /// The ID of the most recent message in the thread or `None` when the thread has no messages.
    pub last_message_id: Option<Id<MessageMarker>>,
    /// The time a message in the thread was most recently pinned or `None` when no message has ever been pinned.
    pub last_pin_timestamp: Option<Timestamp<Iso8601>>,
    /// The slowmode interval in seconds (`0`-`21600`).
    pub rate_limit_per_user: Duration<Seconds>,
    pub flags: ThreadFlags,
    pub thread_metadata: ThreadMetadata,
    /// The number of messages in the thread.
    /// Excludes the first message of a forum post and every deleted message.
    pub message_count: usize,
    /// the number of messages ever sent in the thread.
    /// Like `message_count`, excludes the first messasge of a forum post, but unlike
    /// `message_count`, also counts deleted messages.
    pub total_message_sent: usize,
    /// The number of thread members, max 50.
    pub member_count: usize,
    /// The IDs of the forum tags applied to a post, max 5.
    /// Present only for threads in forums.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<Id<ForumTagMarker>>>,
    /// The IDs of the most recently joined members of a post, newest first, max 8.
    /// Present only for threads in forums, but also not present if sent via the gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_ids_preview: Option<Vec<Id<UserMarker>>>,
    /// The thread member object of the caller.
    /// Present only when the caller is a member of the thread, on `GetChannel`, on `ModifyChannel`, on `thread` of a message,
    /// on `ThreadCreate`, and on the threads list of a guild in `Ready`, `GuildCreate`, and `GuildSync`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<ThreadMember>,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ThreadAutoArchiveDuration {
    OneHour = 60,
    OneDay = 1440,
    ThreeDays = 4320,
    OneWeek = 10080,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ThreadType {
    /// A thread in an announcement channel.
    Announcement = 10,
    Public = 11,
    Private = 12,
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ThreadFlags: u32 {
        const PINNED = 1 << 1;
    }
}

serde_bitflags!(ThreadFlags, u32);

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: ThreadAutoArchiveDuration,
    /// The time the thread was last archived, unarchived, or given a new auto archive duration.
    /// Equal to `create_timestamp` until the first of those changes.
    pub archive_timestamp: Timestamp<Iso8601>,
    /// Whether only thread moderators can act in the thread.
    pub locked: bool,
    /// Whether a member who is not a thread moderator can add members who are not thread moderators.
    /// Only present on a private thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    pub create_timestamp: Timestamp<Iso8601>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ThreadMember {
    /// The ID of the thread.
    /// Omitted on the member of a thread in `Ready`, `GuildCreate`, and `GuildSync`.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<Id<ChannelMarker>>,
    /// Omitted on the member of a thread in `Ready`, `GuildCreate`, and `GuildSync`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Id<UserMarker>>,
    /// The time the user last joined the thread.
    pub join_timestamp: Timestamp<Iso8601>,
    pub flags: ThreadMemberFlags,
    /// Whether the user muted the thread.
    /// Present only on the caller's own membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    ///  In an HTTP response, present only when the request sets `with_member` and the user is still a guild member.
    /// `ThreadMembersUpdate` defines it on each `added_members` entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_config: Option<GuildMuteConfiguration>,
    ///  In an HTTP response, present only when the request sets `with_member` and the user is still a guild member.
    /// `ThreadMembersUpdate` defines it on each `added_members` entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_member: Option<GuildMember>,
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ThreadMemberFlags: u32 {
        const HAS_INTERACTED = 1 << 0;
        /// Notify the user of every message in the thread.
        const ALL_MESSAGES = 1 << 1;
        /// Notify the user only of mentions in the thread.
        const ONLY_MENTIONS = 1 << 2;
        /// Notify the user of nothing in the thread.
        const NO_MESSAGES = 1 << 3;
    }
}

serde_bitflags!(ThreadMemberFlags, u32);
