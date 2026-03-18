use serde::Deserialize;

use crate::{
    channel::message::Message,
    gateway::payload::incoming::{
        audit_log_events::audit_log_entry_create::GuildAuditLogEntryCreate,
        authentication_events::auth_session_change::AuthSessionChange,
        content_events::{
            guild_emojis_update::GuildEmojisUpdate, guild_stickers_update::GuildStickersUpdate,
        },
        favorite_memes_events::favorite_meme_delete::FavoriteMemeDelete,
        guild_events::{guild_create::GuildCreate, guild_delete::GuildDelete},
        members_events::guild_member_remove::GuildMemberRemove,
        message_events::message_create::MessageCreate,
        message_reactions_events::{
            message_reaction_add::MessageReactionAdd,
            message_reaction_remove::MessageReactionRemove,
            message_reaction_remove_all::MessageReactionRemoveAll,
            message_reaction_remove_emoji::MessageReactionRemoveEmoji,
        },
        presence_events::presence_update::PresenceUpdate,
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
    },
    guild::{GuildResponse, member::GuildMember},
    id::{Id, marker::ChannelMarker},
    user::settings::{FavoriteMeme, UserGuildSettings, UserSettings},
};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "t", content = "d", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchEvent {
    Ready(Ready),
    GuildDelete(GuildDelete),
    GuildCreate(GuildCreate),
    TypingStart(TypingStart),
    MessageCreate(MessageCreate),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionRemove(MessageReactionRemove),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmoji),
    MessageReactionRemoveAll(MessageReactionRemoveAll),
    GuildEmojisUpdate(GuildEmojisUpdate),
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
    PresenceUpdate(PresenceUpdate),
    GuildUpdate(GuildResponse),
    /// Sent when a user joins a guild.
    GuildMemberAdd(GuildMember),
    GuildMemberUpdate(GuildMember),
    GuildMemberRemove(GuildMemberRemove),
    GuildRoleCreate(GuildRoleCreate),
    GuildRoleUpdate(GuildRoleUpdate),
    GuildRoleUpdateBulk(GuildRoleUpdateBulk),
    GuildRoleDelete(GuildRoleDelete),
    GuildStickersUpdate(GuildStickersUpdate),
    // TODO: Other variants
}

#[derive(Deserialize, Clone, Debug)]
pub struct DispatchEventPayload {
    #[serde(flatten)]
    pub event: DispatchEvent,
    #[serde(rename = "s")]
    pub sequence_number: u64,
}
