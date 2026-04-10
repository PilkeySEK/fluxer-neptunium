use serde::Deserialize;

use crate::{
    channel::{Channel, message::Message},
    gateway::payload::incoming::{
        AuthSessionChange, CallCreate, CallDelete, CallUpdate, ChannelPinsAck, ChannelPinsUpdate,
        ChannelRecipientAdd, ChannelRecipientRemove, ChannelUpdateBulk, FavoriteMemeDelete,
        GuildAuditLogEntryCreate, GuildBanAdd, GuildBanRemove, GuildCreate, GuildDelete,
        GuildEmojisUpdate, GuildMemberRemove, GuildMembersChunk, GuildRoleCreate, GuildRoleDelete,
        GuildRoleUpdate, GuildRoleUpdateBulk, GuildStickersUpdate, InviteDelete, MessageAck,
        MessageCreate, MessageDelete, MessageDeleteBulk, MessageReactionAdd, MessageReactionRemove,
        MessageReactionRemoveAll, MessageReactionRemoveEmoji, PresenceUpdateIncoming, Ready,
        RecentMentionDelete, RelationshipRemove, SavedMessageDelete, TypingStart, UserNoteUpdate,
        UserPrivateResponse, VoiceServerUpdate, VoiceStateUpdate, WebhooksUpdate,
    },
    guild::{Guild, member::GuildMember},
    id::{Id, marker::ChannelMarker},
    invites::InviteWithMetadata,
    user::{
        relationship::Relationship,
        settings::{FavoriteMeme, UserGuildSettings, UserSettings},
    },
};

// For some reason rust-analyzer tells me a warning but clippy does not, this is
// to make both of them happy
#[allow(clippy::large_enum_variant)]
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "t", content = "d", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchEvent {
    Ready(Ready),
    /// The payload is null. The presence of this event indicates a successful resume.
    Resumed(()),
    // TODO: Find out what type this is
    SessionsReplace(Vec<serde_json::Value>),
    GuildAuditLogEntryCreate(GuildAuditLogEntryCreate),
    UserUpdate(UserPrivateResponse),
    UserPinnedDmsUpdate(Vec<Id<ChannelMarker>>),
    UserSettingsUpdate(UserSettings),
    UserGuildSettingsUpdate(UserGuildSettings),
    UserNoteUpdate(UserNoteUpdate),
    RecentMentionDelete(RecentMentionDelete),
    SavedMessageCreate(Message),
    SavedMessageDelete(SavedMessageDelete),
    FavoriteMemeCreate(FavoriteMeme),
    FavoriteMemeUpdate(FavoriteMeme),
    FavoriteMemeDelete(FavoriteMemeDelete),
    AuthSessionChange(AuthSessionChange),
    PresenceUpdate(PresenceUpdateIncoming),
    GuildCreate(GuildCreate),
    GuildUpdate(Guild),
    GuildSync(GuildCreate),
    GuildDelete(GuildDelete),
    /// Sent when a user joins a guild.
    GuildMemberAdd(GuildMember),
    GuildMemberUpdate(GuildMember),
    GuildMemberRemove(GuildMemberRemove),
    GuildRoleCreate(GuildRoleCreate),
    GuildRoleUpdate(GuildRoleUpdate),
    GuildRoleUpdateBulk(GuildRoleUpdateBulk),
    GuildRoleDelete(GuildRoleDelete),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildStickersUpdate(GuildStickersUpdate),
    GuildBanAdd(GuildBanAdd),
    GuildBanRemove(GuildBanRemove),
    ChannelCreate(Channel),
    ChannelUpdate(Channel),
    ChannelUpdateBulk(ChannelUpdateBulk),
    ChannelDelete(Channel),
    /// Sent when a mesage is pinned or unpinned.
    ChannelPinsUpdate(ChannelPinsUpdate),
    ChannelPinsAck(ChannelPinsAck),
    /// Sent when a user is added to a group DM.
    /// Only existing members of the group DM will receive this message,
    /// the new user receives a `ChannelCreate` event instead.
    ChannelRecipientAdd(ChannelRecipientAdd),
    /// Sent when a user is removed from a group DM.
    ChannelRecipientRemove(ChannelRecipientRemove),
    MessageCreate(MessageCreate),
    MessageUpdate(Message),
    MessageDelete(MessageDelete),
    MessageDeleteBulk(MessageDeleteBulk),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionRemove(MessageReactionRemove),
    MessageReactionRemoveAll(MessageReactionRemoveAll),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmoji),
    MessageAck(MessageAck),
    TypingStart(TypingStart),
    /// Sent when a guild channel webhooks is created, updated, or deleted.
    /// Indicates that webhooks for the channel should be re-fetched.
    WebhooksUpdate(WebhooksUpdate),
    InviteCreate(InviteWithMetadata),
    InviteDelete(InviteDelete),
    RelationshipAdd(Relationship),
    RelationshipUpdate(Relationship),
    RelationshipRemove(RelationshipRemove),
    /// Sent when a user’s voice state is updated (join/leave/move voice channel, mute/deafen).
    VoiceStateUpdate(VoiceStateUpdate),
    VoiceServerUpdate(VoiceServerUpdate),
    CallCreate(CallCreate),
    CallUpdate(CallUpdate),
    // TODO: What do you mean all recipients? Like people who have joined the call at least once or something else?
    /// Dispatched to all recipients when the call terminates (all participants leave or timeout).
    CallDelete(CallDelete),
    // Source: https://github.com/fluxerapp/fluxer/blob/refactor/fluxer_app/src/stores/gateway/handlers/guild/PassiveUpdates.tsx#L39
    PassiveUpdates(crate::gateway::payload::incoming::PassiveUpdates),
    // Source: https://github.com/fluxerapp/fluxer/blob/ee1f27fe1a372b5291aead8042944afd706bf5db/fluxer_app/src/stores/gateway/handlers/guild/GuildMembersChunk.tsx#L45
    /// Sent in response to `RequestGuildMembers`.
    GuildMembersChunk(GuildMembersChunk),
}

#[derive(Deserialize, Clone, Debug)]
pub struct DispatchEventPayload {
    #[serde(flatten)]
    pub event: DispatchEvent,
    #[serde(rename = "s")]
    pub sequence_number: u64,
}
