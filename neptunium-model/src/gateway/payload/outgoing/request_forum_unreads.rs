use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker, MessageMarker},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RequestForumUnreads {
    pub guild_id: Id<GuildMarker>,
    pub channel_id: Id<ChannelMarker>,
    /// Max. `40`.
    pub threads: Vec<ForumUnreadRequestItem>,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct ForumUnreadRequestItem {
    pub thread_id: Id<ChannelMarker>,
    pub ack_message_id: Option<Id<MessageMarker>>,
}
