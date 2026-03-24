use std::sync::Arc;

use async_trait::async_trait;
use bon::Builder;
use neptunium_model::{
    channel::{Channel, message::Message},
    gateway::payload::incoming::{
        audit_log_events::audit_log_entry_create::GuildAuditLogEntryCreate,
        authentication_events::auth_session_change::AuthSessionChange,
        calls_events::{call_create::CallCreate, call_delete::CallDelete, call_update::CallUpdate},
        channel_events::{
            channel_pins_ack::ChannelPinsAck, channel_pins_update::ChannelPinsUpdate,
            channel_update_bulk::ChannelUpdateBulk,
        },
        content_events::{
            guild_emojis_update::GuildEmojisUpdate, guild_stickers_update::GuildStickersUpdate,
        },
        favorite_memes_events::favorite_meme_delete::FavoriteMemeDelete,
        group_dm_events::{
            channel_recipient_add::ChannelRecipientAdd,
            channel_recipient_remove::ChannelRecipientRemove,
        },
        guild_events::{guild_create::GuildCreate, guild_delete::GuildDelete},
        guild_moderation_events::{guild_ban_add::GuildBanAdd, guild_ban_remove::GuildBanRemove},
        invites_events::invite_delete::InviteDelete,
        members_events::guild_member_remove::GuildMemberRemove,
        message_events::{
            message_create::MessageCreate, message_delete::MessageDelete,
            message_delete_bulk::MessageDeleteBulk,
        },
        message_reactions_events::{
            message_reaction_add::MessageReactionAdd,
            message_reaction_remove::MessageReactionRemove,
            message_reaction_remove_all::MessageReactionRemoveAll,
            message_reaction_remove_emoji::MessageReactionRemoveEmoji,
        },
        presence_events::presence_update::PresenceUpdateIncoming,
        read_state_events::message_ack::MessageAck,
        relationship_events::relationship_remove::RelationshipRemove,
        roles_events::{
            guild_role_create::GuildRoleCreate, guild_role_delete::GuildRoleDelete,
            guild_role_update::GuildRoleUpdate, guild_role_update_bulk::GuildRoleUpdateBulk,
        },
        session_events::ready::{Ready, UserPrivateResponse},
        typing_events::typing_start::TypingStart,
        user_content_events::{
            recent_mention_delete::RecentMentionDelete, saved_message_delete::SavedMessageDelete,
        },
        user_events::user_note_update::UserNoteUpdate,
        voice_events::{
            voice_server_update::VoiceServerUpdate, voice_state_update::VoiceStateUpdate,
        },
        webhooks_events::webhooks_update::WebhooksUpdate,
    },
    guild::{GuildResponse, member::GuildMember},
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
    async fn on_ready(&self, ctx: Context, data: Arc<Ready>) -> Result<(), EventError> {
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
        data: Arc<UserPrivateResponse>,
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
        data: Arc<UserSettings>,
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
        data: Arc<Message>,
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
        data: Arc<GuildCreate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_update(
        &self,
        ctx: Context,
        data: Arc<GuildResponse>,
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
        data: Arc<GuildRoleCreate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_role_update(
        &self,
        ctx: Context,
        data: Arc<GuildRoleUpdate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_guild_role_update_bulk(
        &self,
        ctx: Context,
        data: Arc<GuildRoleUpdateBulk>,
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
    async fn on_channel_create(&self, ctx: Context, data: Arc<Channel>) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_update(&self, ctx: Context, data: Arc<Channel>) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_update_bulk(
        &self,
        ctx: Context,
        data: Arc<ChannelUpdateBulk>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_channel_delete(&self, ctx: Context, data: Arc<Channel>) -> Result<(), EventError> {
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
        data: Arc<MessageCreate>,
    ) -> Result<(), EventError> {
        Ok(())
    }
    async fn on_message_update(&self, ctx: Context, data: Arc<Message>) -> Result<(), EventError> {
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
