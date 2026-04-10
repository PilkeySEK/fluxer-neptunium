use std::sync::Arc;

use async_trait::async_trait;
use bon::Builder;
use neptunium_cache_inmemory::{
    Cached, CachedChannel, CachedMessage,
    gateway::cached_payload::{
        CachedGuildCreate, CachedGuildRoleUpdateBulk, CachedMessageCreate, CachedReady,
    },
};
use neptunium_model::{
    gateway::payload::incoming::{
        AuthSessionChange, CallCreate, CallDelete, CallUpdate, ChannelPinsAck, ChannelPinsUpdate,
        ChannelRecipientAdd, ChannelRecipientRemove, ChannelUpdateBulk, FavoriteMemeDelete,
        GuildAuditLogEntryCreate, GuildBanAdd, GuildBanRemove, GuildDelete, GuildEmojisUpdate,
        GuildMemberRemove, GuildMembersChunk, GuildRoleDelete, GuildStickersUpdate, InviteDelete,
        MessageAck, MessageDelete, MessageDeleteBulk, MessageReactionAdd, MessageReactionRemove,
        MessageReactionRemoveAll, MessageReactionRemoveEmoji, PresenceUpdateIncoming,
        RecentMentionDelete, RelationshipRemove, SavedMessageDelete, TypingStart, UserNoteUpdate,
        UserPrivateResponse, VoiceServerUpdate, VoiceStateUpdate, WebhooksUpdate,
    },
    guild::{Guild, member::GuildMember, permissions::GuildRole},
    id::{Id, marker::ChannelMarker},
    invites::InviteWithMetadata,
    user::{
        relationship::Relationship,
        settings::{FavoriteMeme, UserGuildSettings, UserSettings},
    },
};

use crate::events::context::Context;

pub mod context;

#[expect(unused)]
#[async_trait]
pub trait EventHandler: Send {
    async fn on_ready(&self, ctx: Context, data: Arc<CachedReady>) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_resumed(&self, ctx: Context) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_sessions_replace(
        &self,
        ctx: Context,
        data: Arc<Vec<serde_json::Value>>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_audit_log_entry_create(
        &self,
        ctx: Context,
        data: Arc<GuildAuditLogEntryCreate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_user_update(
        &self,
        ctx: Context,
        data: Cached<UserPrivateResponse>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_user_pinned_dms_update(
        &self,
        ctx: Context,
        data: Arc<Vec<Id<ChannelMarker>>>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_user_settings_update(
        &self,
        ctx: Context,
        data: Cached<UserSettings>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_user_guild_settings_update(
        &self,
        ctx: Context,
        data: Arc<UserGuildSettings>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_user_note_update(
        &self,
        ctx: Context,
        data: Arc<UserNoteUpdate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_recent_mention_delete(
        &self,
        ctx: Context,
        data: Arc<RecentMentionDelete>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_saved_message_create(
        &self,
        ctx: Context,
        data: Cached<CachedMessage>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_saved_message_delete(
        &self,
        ctx: Context,
        data: Arc<SavedMessageDelete>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_favorite_meme_create(
        &self,
        ctx: Context,
        data: Arc<FavoriteMeme>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_favorite_meme_update(
        &self,
        ctx: Context,
        data: Arc<FavoriteMeme>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_favorite_meme_delete(
        &self,
        ctx: Context,
        data: Arc<FavoriteMemeDelete>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_auth_session_change(
        &self,
        ctx: Context,
        data: Arc<AuthSessionChange>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_presence_update(
        &self,
        ctx: Context,
        data: Arc<PresenceUpdateIncoming>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_create(
        &self,
        ctx: Context,
        data: Arc<CachedGuildCreate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_update(&self, ctx: Context, data: Cached<Guild>) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_sync(
        &self,
        ctx: Context,
        data: Arc<CachedGuildCreate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_delete(
        &self,
        ctx: Context,
        data: Arc<GuildDelete>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_member_add(
        &self,
        ctx: Context,
        data: Arc<GuildMember>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_member_update(
        &self,
        ctx: Context,
        data: Arc<GuildMember>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_member_remove(
        &self,
        ctx: Context,
        data: Arc<GuildMemberRemove>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_role_create(
        &self,
        ctx: Context,
        data: Cached<GuildRole>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_role_update(
        &self,
        ctx: Context,
        data: Cached<GuildRole>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_role_update_bulk(
        &self,
        ctx: Context,
        data: Arc<CachedGuildRoleUpdateBulk>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_role_delete(
        &self,
        ctx: Context,
        data: Arc<GuildRoleDelete>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_emojis_update(
        &self,
        ctx: Context,
        data: Arc<GuildEmojisUpdate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_stickers_update(
        &self,
        ctx: Context,
        data: Arc<GuildStickersUpdate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_ban_add(
        &self,
        ctx: Context,
        data: Arc<GuildBanAdd>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_ban_remove(
        &self,
        ctx: Context,
        data: Arc<GuildBanRemove>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_create(
        &self,
        ctx: Context,
        data: Cached<CachedChannel>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_update(
        &self,
        ctx: Context,
        data: Cached<CachedChannel>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_update_bulk(
        &self,
        ctx: Context,
        data: Arc<ChannelUpdateBulk>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_delete(
        &self,
        ctx: Context,
        data: Arc<CachedChannel>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_pins_update(
        &self,
        ctx: Context,
        data: Arc<ChannelPinsUpdate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_pins_ack(
        &self,
        ctx: Context,
        data: Arc<ChannelPinsAck>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_recipient_add(
        &self,
        ctx: Context,
        data: Arc<ChannelRecipientAdd>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_recipient_remove(
        &self,
        ctx: Context,
        data: Arc<ChannelRecipientRemove>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_message_create(
        &self,
        ctx: Context,
        data: Arc<CachedMessageCreate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_message_update(
        &self,
        ctx: Context,
        data: Cached<CachedMessage>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_message_delete(
        &self,
        ctx: Context,
        data: Arc<MessageDelete>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_message_delete_bulk(
        &self,
        ctx: Context,
        data: Arc<MessageDeleteBulk>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_message_reaction_add(
        &self,
        ctx: Context,
        data: Arc<MessageReactionAdd>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_message_reaction_remove(
        &self,
        ctx: Context,
        data: Arc<MessageReactionRemove>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_message_reaction_remove_all(
        &self,
        ctx: Context,
        data: Arc<MessageReactionRemoveAll>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_message_reaction_remove_emoji(
        &self,
        ctx: Context,
        data: Arc<MessageReactionRemoveEmoji>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_message_ack(&self, ctx: Context, data: Arc<MessageAck>) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_typing_start(
        &self,
        ctx: Context,
        data: Arc<TypingStart>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_webhooks_update(
        &self,
        ctx: Context,
        data: Arc<WebhooksUpdate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_invite_create(
        &self,
        ctx: Context,
        data: Arc<InviteWithMetadata>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_invite_delete(
        &self,
        ctx: Context,
        data: Arc<InviteDelete>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_relationship_add(
        &self,
        ctx: Context,
        data: Arc<Relationship>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_relationship_update(
        &self,
        ctx: Context,
        data: Arc<Relationship>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_relationship_remove(
        &self,
        ctx: Context,
        data: Arc<RelationshipRemove>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_voice_state_update(
        &self,
        ctx: Context,
        data: Arc<VoiceStateUpdate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_voice_server_update(
        &self,
        ctx: Context,
        data: Arc<VoiceServerUpdate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_call_create(&self, ctx: Context, data: Arc<CallCreate>) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_call_update(&self, ctx: Context, data: Arc<CallUpdate>) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_call_delete(&self, ctx: Context, data: Arc<CallDelete>) -> Result<(), EventError> {
        Ok(())
    }
    // #[cfg(feature = "user_api")]
    async fn on_passive_updates(
        &self,
        ctx: Context,
        data: Arc<neptunium_model::gateway::payload::incoming::PassiveUpdates>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_members_chunk(
        &self,
        ctx: Context,
        data: Arc<GuildMembersChunk>,
    ) -> Result<(), EventError> {
        Ok(())
    }
}

/// An error returned by an event handler.
#[derive(Builder, Debug)]
pub struct EventError {
    /// Whether the error should be propagated to the `Client::start()` caller.
    /// If `true`, the client will be stopped.
    #[builder(default = false)]
    pub propagate: bool,
    /// The kind of error.
    pub kind: EventErrorKind,
}

impl EventError {
    #[must_use]
    pub fn new(kind: EventErrorKind, propagate: bool) -> Self {
        Self { propagate, kind }
    }
}

#[derive(Debug)]
pub enum EventErrorKind {
    ClientError(crate::client::error::Error),
}

// Mainly for use with the `?` operator.
impl From<crate::client::error::Error> for EventError {
    fn from(value: crate::client::error::Error) -> Self {
        Self {
            propagate: false,
            kind: EventErrorKind::ClientError(value),
        }
    }
}

impl std::fmt::Display for EventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            EventErrorKind::ClientError(err) => f.write_fmt(format_args!("Client Error: {err}")),
        }
    }
}
