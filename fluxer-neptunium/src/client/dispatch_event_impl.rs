use neptunium_cache_inmemory::gateway::CachedDispatchEvent;
use neptunium_model::gateway::event::dispatch::DispatchEvent;
use std::sync::Arc;
use tokio::sync::mpsc::error::SendError;

impl super::Client {
    #[expect(clippy::too_many_lines)]
    pub(super) fn handle_dispatch_event(&mut self, event: DispatchEvent) {
        tracing::trace!("Dispatch Event: {event:?}");
        macro_rules! call_event_handlers {
            ($handlers:expr, $ctx:expr, $data:expr => $func_name:ident) => {{
                let arc = Arc::new($data);
                for handler in &$handlers {
                    let handler = Arc::clone(handler);
                    let cloned_arc = Arc::clone(&arc);
                    let ctx_clone = $ctx.clone();
                    // TODO: Maybe store all the `JoinHandle`s in an array in the `Client` struct so that they could all be cancelled
                    // when the `Client` stops? Maybe...
                    tokio::spawn(async move {
                        if let Err(e) = handler.$func_name(ctx_clone, cloned_arc).await {
                            tracing::warn!("Event handler returned error: {e}");
                        }
                    });
                }
            }};
        }
        macro_rules! call_event_handlers_noarc {
            ($handlers:expr, $ctx:expr, $data:expr => $func_name:ident) => {{
                for handler in &$handlers {
                    let handler = Arc::clone(handler);
                    let cloned_data = $data.clone();
                    let ctx_clone = $ctx.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handler.$func_name(ctx_clone, cloned_data).await {
                            tracing::warn!("Event handler returned error: {e}");
                        }
                    });
                }
            }};
        }

        let event = CachedDispatchEvent::from_dispatch_event(event, &self.context.cache);

        match event {
            CachedDispatchEvent::Ready(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_ready);
            }
            CachedDispatchEvent::Resumed(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_resumed);
            }
            CachedDispatchEvent::SessionsReplace(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_sessions_replace);
            }
            CachedDispatchEvent::GuildAuditLogEntryCreate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_audit_log_entry_create);
            }
            CachedDispatchEvent::UserUpdate(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_user_update);
            }
            CachedDispatchEvent::UserPinnedDmsUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_user_pinned_dms_update);
            }
            CachedDispatchEvent::UserSettingsUpdate(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_user_settings_update);
            }
            CachedDispatchEvent::UserGuildSettingsUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_user_guild_settings_update);
            }
            CachedDispatchEvent::UserNoteUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_user_note_update);
            }
            CachedDispatchEvent::RecentMentionDelete(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_recent_mention_delete);
            }
            CachedDispatchEvent::SavedMessageCreate(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_saved_message_create);
            }
            CachedDispatchEvent::SavedMessageDelete(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_saved_message_delete);
            }
            CachedDispatchEvent::SavedMediaCreate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_saved_media_create);
            }
            CachedDispatchEvent::SavedMediaUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_saved_media_update);
            }
            CachedDispatchEvent::SavedMediaDelete(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_saved_media_delete);
            }
            CachedDispatchEvent::AuthSessionChange(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_auth_session_change);
            }
            CachedDispatchEvent::PresenceUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_presence_update);
            }
            CachedDispatchEvent::GuildCreate(data) => {
                #[cfg(feature = "user_api")]
                if self.subscribe_to_everything {
                    let ctx = self.context.clone();
                    let guild_id = data.id;
                    tokio::spawn(async move {
                        use neptunium_model::gateway::payload::outgoing::GuildSubscriptionRequest;

                        let hashmap = std::iter::once((
                            guild_id,
                            GuildSubscriptionRequest {
                                active: Some(true),
                                member_list_channels: None,
                                typing: None,
                                members: None,
                                sync: Some(true),
                            },
                        ))
                        .collect();

                        if let Err(e) = ctx.update_guild_event_subscriptions(hashmap).await {
                            tracing::error!("Failed to subscribe to guild {}: {}", guild_id, e);
                        }
                    });
                }
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_create);
            }
            CachedDispatchEvent::GuildUpdate(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_guild_update);
            }
            CachedDispatchEvent::GuildSync(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_sync);
            }
            CachedDispatchEvent::GuildDelete(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_delete);
            }
            CachedDispatchEvent::GuildMemberAdd(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_guild_member_add);
            }
            CachedDispatchEvent::GuildMemberUpdate(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_guild_member_update);
            }
            CachedDispatchEvent::GuildMemberRemove(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_member_remove);
            }
            CachedDispatchEvent::GuildRoleCreate(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_guild_role_create);
            }
            CachedDispatchEvent::GuildRoleUpdate(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_guild_role_update);
            }
            CachedDispatchEvent::GuildRoleUpdateBulk(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_role_update_bulk);
            }
            CachedDispatchEvent::GuildRoleDelete(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_role_delete);
            }
            CachedDispatchEvent::GuildEmojisUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_emojis_update);
            }
            CachedDispatchEvent::GuildStickersUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_stickers_update);
            }
            CachedDispatchEvent::GuildBanAdd(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_ban_add);
            }
            CachedDispatchEvent::GuildBanRemove(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_ban_remove);
            }
            CachedDispatchEvent::ChannelCreate(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_channel_create);
            }
            CachedDispatchEvent::ChannelUpdate(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_channel_update);
            }
            CachedDispatchEvent::ChannelUpdateBulk(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_channel_update_bulk);
            }
            CachedDispatchEvent::ChannelDelete(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_channel_delete);
            }
            CachedDispatchEvent::ChannelPinsUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_channel_pins_update);
            }
            CachedDispatchEvent::ChannelPinsAck(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_channel_pins_ack);
            }
            CachedDispatchEvent::ChannelRecipientAdd(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_channel_recipient_add);
            }
            CachedDispatchEvent::ChannelRecipientRemove(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_channel_recipient_remove);
            }
            CachedDispatchEvent::MessageCreate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_message_create);
            }
            CachedDispatchEvent::MessageUpdate(data) => {
                call_event_handlers_noarc!(self.event_handlers, self.context, data => on_message_update);
            }
            CachedDispatchEvent::MessageDelete(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_message_delete);
            }
            CachedDispatchEvent::MessageDeleteBulk(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_message_delete_bulk);
            }
            CachedDispatchEvent::MessageReactionAdd(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_message_reaction_add);
            }
            CachedDispatchEvent::MessageReactionRemove(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_message_reaction_remove);
            }
            CachedDispatchEvent::MessageReactionRemoveAll(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_message_reaction_remove_all);
            }
            CachedDispatchEvent::MessageReactionRemoveEmoji(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_message_reaction_remove_emoji);
            }
            CachedDispatchEvent::MessageAck(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_message_ack);
            }
            CachedDispatchEvent::TypingStart(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_typing_start);
            }
            CachedDispatchEvent::WebhooksUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_webhooks_update);
            }
            CachedDispatchEvent::InviteCreate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_invite_create);
            }
            CachedDispatchEvent::InviteDelete(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_invite_delete);
            }
            CachedDispatchEvent::RelationshipAdd(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_relationship_add);
            }
            CachedDispatchEvent::RelationshipUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_relationship_update);
            }
            CachedDispatchEvent::RelationshipRemove(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_relationship_remove);
            }
            CachedDispatchEvent::VoiceStateUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_voice_state_update);
            }
            CachedDispatchEvent::VoiceServerUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_voice_server_update);
            }
            CachedDispatchEvent::CallCreate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_call_create);
            }
            CachedDispatchEvent::CallUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_call_update);
            }
            CachedDispatchEvent::CallDelete(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_call_delete);
            }
            // #[cfg(feature = "user_api")]
            CachedDispatchEvent::PassiveUpdates(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_passive_updates);
            }
            CachedDispatchEvent::GuildMembersChunk(data) => {
                // Unfortunately it seems that there is no easy way to avoid cloning the nonce here,
                // else rust will give an error that part of `data` is borrowed.
                if let Some(nonce) = data.nonce.clone()
                    && let Some(listener_tx) = self.guild_members_chunk_listeners.get(&nonce)
                {
                    // The receiving end being dropped indicates that it has received all the messages it needs.
                    // However, receiving the same nonce is concerning as it likely indicates a bug with how it
                    // detects whether all messages have been received.
                    if let Err(SendError(data)) = listener_tx.send(data) {
                        tracing::warn!(
                            "Guild member chunk with nonce \"{}\" is managed but the receiver has been dropped.",
                            nonce
                        );
                        self.guild_members_chunk_listeners.remove(&nonce);
                        call_event_handlers!(self.event_handlers, self.context, data => on_guild_members_chunk);
                    }
                } else {
                    call_event_handlers!(self.event_handlers, self.context, data => on_guild_members_chunk);
                }
            }
            CachedDispatchEvent::GuildMemberListUpdate(data) => {
                call_event_handlers!(self.event_handlers, self.context, data => on_guild_member_list_update);
            }
            CachedDispatchEvent::GuildCountsUpdate(data) => {
                let maybe_data = if let Some(nonce) = &data.nonce {
                    if let Some(listener_tx) = self.guild_counts_update_listeners.remove(nonce) {
                        // If this returns an error it means that the future listening to the response has been cancelled.
                        listener_tx.send(data).err()
                    } else {
                        Some(data)
                    }
                } else {
                    Some(data)
                };
                if let Some(data) = maybe_data {
                    call_event_handlers!(self.event_handlers, self.context, data => on_guild_counts_update);
                }
            }
        }
    }
}
