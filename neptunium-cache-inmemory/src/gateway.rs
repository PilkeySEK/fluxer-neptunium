use std::sync::Arc;

use neptunium_model::{
    channel::Channel,
    gateway::{
        event::dispatch::DispatchEvent,
        payload::incoming::{
            AuthSessionChange, CallCreate, CallDelete, CallUpdate, ChannelPinsAck,
            ChannelPinsUpdate, ChannelRecipientAdd, ChannelRecipientRemove, ChannelUpdateBulk,
            FavoriteMemeDelete, GuildAuditLogEntryCreate, GuildBanAdd, GuildBanRemove, GuildCreate,
            GuildDelete, GuildEmojisUpdate, GuildMemberRemove, GuildRoleCreate, GuildRoleDelete,
            GuildRoleUpdate, GuildRoleUpdateBulk, GuildStickersUpdate, InviteDelete, MessageAck,
            MessageDelete, MessageDeleteBulk, MessageReactionAdd, MessageReactionRemove,
            MessageReactionRemoveAll, MessageReactionRemoveEmoji, PresenceUpdateIncoming,
            RecentMentionDelete, RelationshipRemove, SavedMessageDelete, TypingStart,
            UserNoteUpdate, UserPrivateResponse, VoiceServerUpdate, VoiceStateUpdate,
            WebhooksUpdate,
        },
    },
    guild::{Guild, member::GuildMember},
    id::{Id, marker::ChannelMarker},
    invites::InviteWithMetadata,
    user::{
        relationship::Relationship,
        settings::{FavoriteMeme, UserGuildSettings, UserSettings},
    },
};

use crate::{
    Cache, CacheValue, Cached, CachedChannel, CachedMessage,
    gateway::cached_payload::{CachedMessageCreate, CachedReady, FromNonCached},
};

pub mod cached_payload;

impl CachedDispatchEvent {
    #[expect(clippy::too_many_lines)]
    pub async fn from_dispatch_event(event: DispatchEvent, cache: &Arc<Cache>) -> Self {
        match event {
            DispatchEvent::Ready(payload) => {
                CachedDispatchEvent::Ready(CachedReady::from_noncached(payload, cache).await)
            }
            DispatchEvent::Resumed(()) => CachedDispatchEvent::Resumed(()),
            DispatchEvent::SessionsReplace(payload) => {
                CachedDispatchEvent::SessionsReplace(payload)
            }
            DispatchEvent::GuildAuditLogEntryCreate(payload) => {
                CachedDispatchEvent::GuildAuditLogEntryCreate(payload)
            }
            DispatchEvent::UserUpdate(payload) => {
                CachedDispatchEvent::UserUpdate(payload.insert_and_return(cache).await)
            }
            DispatchEvent::UserPinnedDmsUpdate(payload) => {
                CachedDispatchEvent::UserPinnedDmsUpdate(payload)
            }
            DispatchEvent::UserSettingsUpdate(payload) => {
                CachedDispatchEvent::UserSettingsUpdate(payload.insert_and_return(cache).await)
            }
            DispatchEvent::UserGuildSettingsUpdate(payload) => {
                CachedDispatchEvent::UserGuildSettingsUpdate(payload)
            }
            DispatchEvent::UserNoteUpdate(payload) => CachedDispatchEvent::UserNoteUpdate(payload),
            DispatchEvent::RecentMentionDelete(payload) => {
                CachedDispatchEvent::RecentMentionDelete(payload)
            }
            DispatchEvent::SavedMessageCreate(payload) => CachedDispatchEvent::SavedMessageCreate(
                CachedMessage::from_message(payload, cache)
                    .await
                    .insert_and_return(cache)
                    .await,
            ),
            DispatchEvent::SavedMessageDelete(payload) => {
                CachedDispatchEvent::SavedMessageDelete(payload)
            }
            DispatchEvent::FavoriteMemeCreate(payload) => {
                CachedDispatchEvent::FavoriteMemeCreate(payload)
            }
            DispatchEvent::FavoriteMemeUpdate(payload) => {
                CachedDispatchEvent::FavoriteMemeUpdate(payload)
            }
            DispatchEvent::FavoriteMemeDelete(payload) => {
                CachedDispatchEvent::FavoriteMemeDelete(payload)
            }
            DispatchEvent::AuthSessionChange(payload) => {
                CachedDispatchEvent::AuthSessionChange(payload)
            }
            DispatchEvent::PresenceUpdate(payload) => CachedDispatchEvent::PresenceUpdate(payload),
            DispatchEvent::GuildCreate(payload) => CachedDispatchEvent::GuildCreate(payload),
            DispatchEvent::GuildUpdate(payload) => {
                CachedDispatchEvent::GuildUpdate(payload.insert_and_return(cache).await)
            }
            DispatchEvent::GuildDelete(payload) => CachedDispatchEvent::GuildDelete(payload),
            DispatchEvent::GuildMemberAdd(payload) => CachedDispatchEvent::GuildMemberAdd(payload),
            DispatchEvent::GuildMemberUpdate(payload) => {
                CachedDispatchEvent::GuildMemberUpdate(payload)
            }
            DispatchEvent::GuildMemberRemove(payload) => {
                CachedDispatchEvent::GuildMemberRemove(payload)
            }
            DispatchEvent::GuildRoleCreate(payload) => {
                CachedDispatchEvent::GuildRoleCreate(payload)
            }
            DispatchEvent::GuildRoleUpdate(payload) => {
                CachedDispatchEvent::GuildRoleUpdate(payload)
            }
            DispatchEvent::GuildRoleUpdateBulk(payload) => {
                CachedDispatchEvent::GuildRoleUpdateBulk(payload)
            }
            DispatchEvent::GuildRoleDelete(payload) => {
                CachedDispatchEvent::GuildRoleDelete(payload)
            }
            DispatchEvent::GuildEmojisUpdate(payload) => {
                CachedDispatchEvent::GuildEmojisUpdate(payload)
            }
            DispatchEvent::GuildStickersUpdate(payload) => {
                CachedDispatchEvent::GuildStickersUpdate(payload)
            }
            DispatchEvent::GuildBanAdd(payload) => CachedDispatchEvent::GuildBanAdd(payload),
            DispatchEvent::GuildBanRemove(payload) => CachedDispatchEvent::GuildBanRemove(payload),
            DispatchEvent::ChannelCreate(payload) => CachedDispatchEvent::ChannelCreate(
                CachedChannel::from_channel(payload, cache)
                    .await
                    .insert_and_return(cache)
                    .await,
            ),
            DispatchEvent::ChannelUpdate(payload) => CachedDispatchEvent::ChannelUpdate(
                CachedChannel::from_channel(payload, cache)
                    .await
                    .insert_and_return(cache)
                    .await,
            ),
            DispatchEvent::ChannelUpdateBulk(payload) => {
                CachedDispatchEvent::ChannelUpdateBulk(payload)
            }
            DispatchEvent::ChannelDelete(payload) => CachedDispatchEvent::ChannelDelete(payload),
            DispatchEvent::ChannelPinsUpdate(payload) => {
                CachedDispatchEvent::ChannelPinsUpdate(payload)
            }
            DispatchEvent::ChannelPinsAck(payload) => CachedDispatchEvent::ChannelPinsAck(payload),
            DispatchEvent::ChannelRecipientAdd(payload) => {
                CachedDispatchEvent::ChannelRecipientAdd(payload)
            }
            DispatchEvent::ChannelRecipientRemove(payload) => {
                CachedDispatchEvent::ChannelRecipientRemove(payload)
            }
            DispatchEvent::MessageCreate(payload) => CachedDispatchEvent::MessageCreate(
                CachedMessageCreate::from_noncached(payload, cache).await,
            ),
            DispatchEvent::MessageUpdate(payload) => CachedDispatchEvent::MessageUpdate(
                CachedMessage::from_message(payload, cache)
                    .await
                    .insert_and_return(cache)
                    .await,
            ),
            DispatchEvent::MessageDelete(payload) => CachedDispatchEvent::MessageDelete(payload),
            DispatchEvent::MessageDeleteBulk(payload) => {
                CachedDispatchEvent::MessageDeleteBulk(payload)
            }
            DispatchEvent::MessageReactionAdd(payload) => {
                CachedDispatchEvent::MessageReactionAdd(payload)
            }
            DispatchEvent::MessageReactionRemove(payload) => {
                CachedDispatchEvent::MessageReactionRemove(payload)
            }
            DispatchEvent::MessageReactionRemoveAll(payload) => {
                CachedDispatchEvent::MessageReactionRemoveAll(payload)
            }
            DispatchEvent::MessageReactionRemoveEmoji(payload) => {
                CachedDispatchEvent::MessageReactionRemoveEmoji(payload)
            }
            DispatchEvent::MessageAck(payload) => CachedDispatchEvent::MessageAck(payload),
            DispatchEvent::TypingStart(payload) => CachedDispatchEvent::TypingStart(payload),
            DispatchEvent::WebhooksUpdate(payload) => CachedDispatchEvent::WebhooksUpdate(payload),
            DispatchEvent::InviteCreate(payload) => CachedDispatchEvent::InviteCreate(payload),
            DispatchEvent::InviteDelete(payload) => CachedDispatchEvent::InviteDelete(payload),
            DispatchEvent::RelationshipAdd(payload) => {
                CachedDispatchEvent::RelationshipAdd(payload)
            }
            DispatchEvent::RelationshipUpdate(payload) => {
                CachedDispatchEvent::RelationshipUpdate(payload)
            }
            DispatchEvent::RelationshipRemove(payload) => {
                CachedDispatchEvent::RelationshipRemove(payload)
            }
            DispatchEvent::VoiceStateUpdate(payload) => {
                CachedDispatchEvent::VoiceStateUpdate(payload)
            }
            DispatchEvent::VoiceServerUpdate(payload) => {
                CachedDispatchEvent::VoiceServerUpdate(payload)
            }
            DispatchEvent::CallCreate(payload) => CachedDispatchEvent::CallCreate(payload),
            DispatchEvent::CallUpdate(payload) => CachedDispatchEvent::CallUpdate(payload),
            DispatchEvent::CallDelete(payload) => CachedDispatchEvent::CallDelete(payload),
            DispatchEvent::PassiveUpdates(payload) => CachedDispatchEvent::PassiveUpdates(payload),
        }
    }
}

pub enum CachedDispatchEvent {
    Ready(CachedReady),
    /// The payload is null. The presence of this event indicates a successful resume.
    Resumed(()),
    SessionsReplace(Vec<serde_json::Value>),
    GuildAuditLogEntryCreate(GuildAuditLogEntryCreate),
    UserUpdate(Cached<UserPrivateResponse>),
    UserPinnedDmsUpdate(Vec<Id<ChannelMarker>>),
    UserSettingsUpdate(Cached<UserSettings>),
    UserGuildSettingsUpdate(UserGuildSettings),
    UserNoteUpdate(UserNoteUpdate),
    RecentMentionDelete(RecentMentionDelete),
    SavedMessageCreate(Cached<CachedMessage>),
    SavedMessageDelete(SavedMessageDelete),
    FavoriteMemeCreate(FavoriteMeme),
    FavoriteMemeUpdate(FavoriteMeme),
    FavoriteMemeDelete(FavoriteMemeDelete),
    AuthSessionChange(AuthSessionChange),
    PresenceUpdate(PresenceUpdateIncoming),
    GuildCreate(GuildCreate),
    GuildUpdate(Cached<Guild>),
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
    ChannelCreate(Cached<CachedChannel>),
    ChannelUpdate(Cached<CachedChannel>),
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
    MessageCreate(CachedMessageCreate),
    MessageUpdate(Cached<CachedMessage>),
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
    /// Dispatched to all recipients when the call terminates (all participants leave or timeout).
    CallDelete(CallDelete),
    PassiveUpdates(neptunium_model::gateway::payload::incoming::PassiveUpdates),
}
