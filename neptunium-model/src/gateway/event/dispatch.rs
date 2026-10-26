use serde::{Deserialize, Serialize};

use crate::{
    channel::{Channel, message::Message, thread::ThreadChannel},
    gateway::payload::incoming::{
        AuthSessionChange, CallCreate, CallDelete, CallUpdate, ChannelMemberCountsUpdate,
        ChannelPinsAck, ChannelPinsUpdate, ChannelRecipientAdd, ChannelRecipientRemove,
        ChannelUpdateBulk, EntranceSoundPlay, ForumUnreads, GuildAuditLogEntryCreate, GuildBanAdd,
        GuildBanRemove, GuildCountsUpdate, GuildCreate, GuildDelete, GuildEmojisUpdate,
        GuildMemberAdd, GuildMemberListUpdate, GuildMemberRemove, GuildMemberUpdate,
        GuildMembersChunk, GuildRoleCreate, GuildRoleDelete, GuildRoleUpdate, GuildRoleUpdateBulk,
        GuildStickersUpdate, InviteDelete, MessageAck, MessageCreate, MessageDelete,
        MessageDeleteBulk, MessageReactionAdd, MessageReactionAddMany, MessageReactionRemove,
        MessageReactionRemoveAll, MessageReactionRemoveEmoji, PresenceUpdateBulk,
        PresenceUpdateIncoming, RateLimited, Ready, RecentMentionDelete, RelationshipRemove,
        Resumed, SavedMediaDelete, SavedMessageDelete, SessionPresence, ThreadCreate, ThreadDelete,
        ThreadListSync, ThreadMemberListUpdate, ThreadMemberUpdate, ThreadMembersUpdate,
        TypingStart, UserConnectionsUpdate, UserNoteUpdate, UserPrivateResponse, VoiceServerUpdate,
        VoiceSignal, VoiceStateUpdate, WebhooksUpdate,
    },
    guild::Guild,
    id::{Id, marker::ChannelMarker},
    invites::InviteWithMetadata,
    user::{
        auth::WebAuthnCredential,
        relationship::Relationship,
        settings::{SavedMedia, UserGuildSettings, UserSettings},
    },
};

// For some reason rust-analyzer tells me a warning but clippy does not, this is
// to make both of them happy
#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "t", content = "d", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchEvent {
    Ready(Ready),
    /// The presence of this event indicates a successful resume.
    Resumed(Resumed),
    /// The account’s live sessions changed.
    /// Replaces the `sessions` list from `Ready`.
    SessionsReplace(Vec<SessionPresence>),
    AuthSessionChange(AuthSessionChange),
    RateLimited(RateLimited),
    UserUpdate(UserPrivateResponse),
    UserSettingsUpdate(UserSettings),
    UserGuildSettingsUpdate(UserGuildSettings),
    UserNoteUpdate(UserNoteUpdate),
    UserPinnedDmsUpdate(Vec<Id<ChannelMarker>>),
    UserConnectionsUpdate(UserConnectionsUpdate),
    /// Replaces `webauthn_credentials` from `Ready`.
    #[serde(rename = "WEBAUTHN_CREDENTIALS_UPDATE")]
    WebAuthnCredentialsUpdate(Vec<WebAuthnCredential>),
    RelationshipAdd(Relationship),
    RelationshipUpdate(Relationship),
    RelationshipRemove(RelationshipRemove),
    SavedMessageCreate(Message),
    SavedMessageDelete(SavedMessageDelete),
    RecentMentionDelete(RecentMentionDelete),
    #[serde(rename = "FAVORITE_MEME_CREATE")]
    SavedMediaCreate(SavedMedia),
    #[serde(rename = "FAVORITE_MEME_UPDATE")]
    SavedMediaUpdate(SavedMedia),
    #[serde(rename = "FAVORITE_MEME_DELETE")]
    SavedMediaDelete(SavedMediaDelete),
    GuildCreate(GuildCreate),
    /// A subscribed session receives the guild’s full current state again.
    GuildSync(GuildCreate),
    GuildUpdate(Guild),
    GuildDelete(GuildDelete),
    GuildRoleCreate(GuildRoleCreate),
    GuildRoleUpdate(GuildRoleUpdate),
    GuildRoleUpdateBulk(GuildRoleUpdateBulk),
    GuildRoleDelete(GuildRoleDelete),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildStickersUpdate(GuildStickersUpdate),
    ChannelCreate(Channel),
    ChannelUpdate(Channel),
    ChannelUpdateBulk(ChannelUpdateBulk),
    ChannelDelete(Channel),
    /// Sent when a user is added to a group DM.
    /// Only existing members of the group DM will receive this message,
    /// the new user receives a `ChannelCreate` event instead.
    ChannelRecipientAdd(ChannelRecipientAdd),
    /// Sent when a user is removed from a group DM.
    ChannelRecipientRemove(ChannelRecipientRemove),
    /// Sent when a guild channel webhooks is created, updated, or deleted.
    /// Indicates that webhooks for the channel should be re-fetched.
    WebhooksUpdate(WebhooksUpdate),
    InviteCreate(InviteWithMetadata),
    InviteDelete(InviteDelete),
    /// A passive session only receives this event when its user is a member of the thread.
    ThreadCreate(ThreadCreate),
    /// A passive session only receives this event when its user is a member of the thread.
    ThreadUpdate(ThreadChannel),
    /// A passive session only receives this event when its user is a member of the thread.
    ThreadDelete(ThreadDelete),
    /// The session’s active threads are replaced for a guild or for some of its channels.
    ///
    /// `Resume` does not replay this event.
    ThreadListSync(ThreadListSync),
    /// The session’s own membership in a thread changes.
    /// Every session of the user that can view the thread receives this event.
    ThreadMemberUpdate(ThreadMemberUpdate),
    /// Users join or leave a thread.
    ThreadMembersUpdate(ThreadMembersUpdate),
    /// The full member list of a thread the session subscribed to with `thread_member_lists`.
    ///
    /// `Resume` does not replay this event.
    ThreadMemberListUpdate(ThreadMemberListUpdate),
    /// The answer to `RequestForumUnreads`.
    ForumUnreads(ForumUnreads),
    GuildMemberAdd(GuildMemberAdd),
    GuildMemberUpdate(GuildMemberUpdate),
    GuildMemberRemove(GuildMemberRemove),
    // Source: https://github.com/fluxerapp/fluxer/blob/ee1f27fe1a372b5291aead8042944afd706bf5db/fluxer_app/src/stores/gateway/handlers/guild/GuildMembersChunk.tsx#L45
    /// Sent in response to `RequestGuildMembers`.
    GuildMembersChunk(GuildMembersChunk),
    GuildMemberListUpdate(GuildMemberListUpdate),
    GuildAuditLogEntryCreate(GuildAuditLogEntryCreate),
    GuildBanAdd(GuildBanAdd),
    GuildBanRemove(GuildBanRemove),
    PresenceUpdate(PresenceUpdateIncoming),
    PresenceUpdateBulk(PresenceUpdateBulk),
    PassiveUpdates(crate::gateway::payload::incoming::PassiveUpdates),
    MessageCreate(MessageCreate),
    MessageUpdate(Message),
    MessageDelete(MessageDelete),
    MessageDeleteBulk(MessageDeleteBulk),
    /// The current user's read state advances for a channel.
    MessageAck(MessageAck),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionAddMany(MessageReactionAddMany),
    MessageReactionRemove(MessageReactionRemove),
    MessageReactionRemoveAll(MessageReactionRemoveAll),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmoji),
    TypingStart(TypingStart),
    /// Sent when a mesage is pinned or unpinned.
    ChannelPinsUpdate(ChannelPinsUpdate),
    ChannelPinsAck(ChannelPinsAck),
    /// Sent when a user’s voice state is updated (join/leave/move voice channel, mute/deafen).
    VoiceStateUpdate(VoiceStateUpdate),
    VoiceServerUpdate(VoiceServerUpdate),
    /// A signaling message from another peer-to-peer connection.
    VoiceSignal(VoiceSignal),
    /// A participant's entrance sound plays in a voice channel.
    EntranceSoundPlay(EntranceSoundPlay),
    CallCreate(CallCreate),
    CallUpdate(CallUpdate),
    /// A call ends or becomes unavailable.
    CallDelete(CallDelete),
    GuildCountsUpdate(GuildCountsUpdate),
    /// Per-channel member and online counts are returned for one guild.
    ChannelMemberCountsUpdate(ChannelMemberCountsUpdate),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DispatchEventPayload {
    #[serde(flatten)]
    pub event: DispatchEvent,
    #[serde(rename = "s")]
    pub sequence_number: u64,
}
