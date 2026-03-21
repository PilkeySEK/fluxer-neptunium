use serde::Deserialize;

use crate::{
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
    GuildUpdate(GuildResponse),
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
    // TODO: PASSIVE_UPDATES: https://github.com/fluxerapp/fluxer/blob/refactor/fluxer_app/src/stores/gateway/handlers/guild/PassiveUpdates.tsx#L39
    // and make a user_api feature for this
}

#[derive(Deserialize, Clone, Debug)]
pub struct DispatchEventPayload {
    #[serde(flatten)]
    pub event: DispatchEvent,
    #[serde(rename = "s")]
    pub sequence_number: u64,
}
