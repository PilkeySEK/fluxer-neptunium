use async_trait::async_trait;
use neptunium_cache_inmemory::{
    CachableEndpoint, CacheValue, Cached, CachedChannel, CachedMessage,
};
use neptunium_http::endpoints::{
    channel::{
        AddUserToGroupDm, BulkDeleteMessages, CallEligibilityStatus, ChannelSettingsUpdates,
        ChannelSlowmodeInformation, CreateAttachmentsInChannel,
        CreateAttachmentsInChannelAttachment, CreateAttachmentsInChannelResponse, CreateMessage,
        CreateMessageBody, DeleteChannel, DeletePermissionOverwrite, GetCallEligibilityStatus,
        GetChannel, GetChannelSlowmodeInformation, IndicateTyping, ListChannelMessages,
        ListChannelMessagesParams, ListRtcRegions, ListRtcRegionsResponseEntry,
        PermissionOverwriteUpdate, PinDirectMessageChannel, RemoveUserFromGroupDm,
        RingCallRecipients, SetPermissionOverwrite, StopRingingCallRecipients,
        UnpinDirectMessageChannel, UpdateCallRegion, UpdateChannelSettings,
    },
    invites::{CreateChannelInvite, CreateChannelInviteOptions, ListChannelInvites},
    webhooks::{CreateWebhook, ListChannelWebhooks},
};
use neptunium_model::{
    channel::{Channel, VoiceRegion},
    guild::webhook::Webhook,
    id::{
        Id,
        marker::{ChannelMarker, GenericMarker, MessageMarker, UserMarker},
    },
    invites::InviteWithMetadata,
};

use crate::{
    client::error::ClientError,
    events::context::{ApplyDefaultAllowedMentions, Context},
    exts::PartialUserExt,
    internal::traits::channel::ChannelTrait,
};

#[async_trait]
pub trait ChannelExt {
    async fn delete(&self, ctx: &Context) -> Result<(), ClientError>;
    async fn delete_silent(&self, ctx: &Context) -> Result<(), ClientError>;
    // TODO: Maybe make a builder or something around the ChannelSettingsUpdates
    // because it's annoying to create ig
    async fn update_settings(
        &self,
        ctx: &Context,
        settings: ChannelSettingsUpdates,
    ) -> Result<Cached<CachedChannel>, ClientError>;
    async fn get(&self, ctx: &Context) -> Result<Cached<CachedChannel>, ClientError>;
    async fn get_call_eligibility_status(
        &self,
        ctx: &Context,
    ) -> Result<CallEligibilityStatus, ClientError>;
    /// Update the voice region for an ongoing call.
    async fn update_call_region(
        &self,
        ctx: &Context,
        region: VoiceRegion,
    ) -> Result<(), ClientError>;
    /// Sends ringing notifications to specfied users in a call. If the recipients
    /// are set to `None`, rings all channel members.
    async fn ring_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), ClientError>;
    /// Stops ringing notifications for specified users in a call. This allows callers
    /// to stop notifying users who have declined or not responded. Pass `None` for the
    /// recipients to stop ringing everyone.
    async fn stop_ringing_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), ClientError>;
    async fn list_messages(
        &self,
        ctx: &Context,
        params: ListChannelMessagesParams,
    ) -> Result<Vec<Cached<CachedMessage>>, ClientError>;
    async fn bulk_delete_messages(
        &self,
        ctx: &Context,
        messages: Vec<Id<MessageMarker>>,
    ) -> Result<(), ClientError>;
    /// Same as `create_message`.
    async fn send_message(
        &self,
        ctx: &Context,
        message: impl Into<CreateMessageBody> + Send,
    ) -> Result<Cached<CachedMessage>, ClientError>;
    async fn create_message(
        &self,
        ctx: &Context,
        message: impl Into<CreateMessageBody> + Send,
    ) -> Result<Cached<CachedMessage>, ClientError>;
    async fn set_permission_overwrite(
        &self,
        ctx: &Context,
        update: PermissionOverwriteUpdate,
    ) -> Result<(), ClientError>;
    async fn delete_permission_overwrite(
        &self,
        ctx: &Context,
        overwrite_id: Id<GenericMarker>,
    ) -> Result<(), ClientError>;
    #[cfg(feature = "user_api")]
    async fn acknowledge_new_pin_notifications(&self, ctx: &Context) -> Result<(), ClientError>;
    async fn add_user_to_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
    ) -> Result<(), ClientError>;
    /// Remove a user from a group DM or leave a group DM by specifying
    /// your own user ID. Set `silent` to `true` to suppress the system
    /// message when leaving.
    async fn remove_user_from_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
        silent: bool,
    ) -> Result<(), ClientError>;
    async fn list_rtc_regions(
        &self,
        ctx: &Context,
    ) -> Result<Vec<ListRtcRegionsResponseEntry>, ClientError>;
    async fn indicate_typing(&self, ctx: &Context) -> Result<(), ClientError>;
    async fn create_invite(
        &self,
        ctx: &Context,
        options: CreateChannelInviteOptions,
    ) -> Result<Cached<InviteWithMetadata>, ClientError>;
    async fn list_invites(
        &self,
        ctx: &Context,
    ) -> Result<Vec<Cached<InviteWithMetadata>>, ClientError>;
    async fn list_webhooks(&self, ctx: &Context) -> Result<Vec<Webhook>, ClientError>;
    /// Create a webhook in this channel, with the given name and optionally the avatar image as a base64-encoded data URI.
    async fn create_webhook(
        &self,
        ctx: &Context,
        name: String,
        avatar: Option<String>,
    ) -> Result<Webhook, ClientError>;
    /// Pin this channel for the current user if it is a DM channel.
    async fn pin(&self, ctx: &Context) -> Result<(), ClientError>;
    /// Unpin this channel for the current user if it is a DM channel.
    async fn unpin(&self, ctx: &Context) -> Result<(), ClientError>;
    /// Get slowmode information for this channel and the current user, including when the
    /// current user may send the next message.
    async fn get_slowmode_information(
        &self,
        ctx: &Context,
    ) -> Result<ChannelSlowmodeInformation, ClientError>;
    async fn create_attachments(
        &self,
        ctx: &Context,
        attachments: Vec<CreateAttachmentsInChannelAttachment>,
    ) -> Result<CreateAttachmentsInChannelResponse, ClientError>;
}

pub trait ChannelDataExt {
    /// Get the channel name. If this channel doesn't have a `name` set (as is the case with most DM channels),
    /// this will return the name of the DM channel just like how it would be displayed in the Fluxer client.
    /// If the channel has no recipients and no name (should never happen), this will return the channel ID as a string.
    fn channel_name(&self) -> String;
}

impl ChannelDataExt for Channel {
    fn channel_name(&self) -> String {
        if let Some(name) = &self.name {
            name.clone()
        } else if let Some(recipients) = &self.recipients {
            recipients
                .iter()
                .map(PartialUserExt::display_name)
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            self.id.to_string()
        }
    }
}

impl ChannelDataExt for CachedChannel {
    fn channel_name(&self) -> String {
        if let Some(name) = &self.name {
            name.clone()
        } else if let Some(recipients) = &self.recipients {
            recipients
                .iter()
                .map(|recipient| recipient.load().display_name())
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            self.id.to_string()
        }
    }
}

#[async_trait]
impl<T: ChannelTrait> ChannelExt for T {
    async fn delete(&self, ctx: &Context) -> Result<(), ClientError> {
        Ok(DeleteChannel {
            channel_id: self.get_channel_id(),
            silent: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn delete_silent(&self, ctx: &Context) -> Result<(), ClientError> {
        Ok(DeleteChannel {
            channel_id: self.get_channel_id(),
            silent: Some(true),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_settings(
        &self,
        ctx: &Context,
        settings: ChannelSettingsUpdates,
    ) -> Result<Cached<CachedChannel>, ClientError> {
        Ok(UpdateChannelSettings {
            channel_id: self.get_channel_id(),
            updates: settings,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn get(&self, ctx: &Context) -> Result<Cached<CachedChannel>, ClientError> {
        Ok(GetChannel {
            channel_id: self.get_channel_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn get_call_eligibility_status(
        &self,
        ctx: &Context,
    ) -> Result<CallEligibilityStatus, ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(
                GetCallEligibilityStatus::builder()
                    .channel_id(self.get_channel_id())
                    .build(),
            )
            .await?)
    }

    async fn update_call_region(
        &self,
        ctx: &Context,
        region: VoiceRegion,
    ) -> Result<(), ClientError> {
        Ok(UpdateCallRegion {
            channel_id: self.get_channel_id(),
            region,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn ring_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(
                RingCallRecipients::builder()
                    .channel_id(self.get_channel_id())
                    .maybe_recipients(recipients)
                    .build(),
            )
            .await?)
    }

    async fn stop_ringing_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(
                StopRingingCallRecipients::builder()
                    .channel_id(self.get_channel_id())
                    .maybe_recipients(recipients)
                    .build(),
            )
            .await?)
    }

    async fn list_messages(
        &self,
        ctx: &Context,
        params: ListChannelMessagesParams,
    ) -> Result<Vec<Cached<CachedMessage>>, ClientError> {
        Ok(ListChannelMessages {
            channel_id: self.get_channel_id(),
            params,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn bulk_delete_messages(
        &self,
        ctx: &Context,
        messages: Vec<Id<MessageMarker>>,
    ) -> Result<(), ClientError> {
        Ok(BulkDeleteMessages {
            channel_id: self.get_channel_id(),
            messages,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn send_message(
        &self,
        ctx: &Context,
        message: impl Into<CreateMessageBody> + Send,
    ) -> Result<Cached<CachedMessage>, ClientError> {
        self.create_message(ctx, message).await
    }

    async fn create_message(
        &self,
        ctx: &Context,
        message: impl Into<CreateMessageBody> + Send,
    ) -> Result<Cached<CachedMessage>, ClientError> {
        Ok(CreateMessage {
            channel_id: self.get_channel_id(),
            message: message.into().apply_default_allowed_mentions(ctx),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn set_permission_overwrite(
        &self,
        ctx: &Context,
        update: PermissionOverwriteUpdate,
    ) -> Result<(), ClientError> {
        Ok(SetPermissionOverwrite {
            channel_id: self.get_channel_id(),
            overwrite: update,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn delete_permission_overwrite(
        &self,
        ctx: &Context,
        overwrite_id: Id<GenericMarker>,
    ) -> Result<(), ClientError> {
        Ok(DeletePermissionOverwrite {
            channel_id: self.get_channel_id(),
            overwrite_id,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    #[cfg(feature = "user_api")]
    async fn acknowledge_new_pin_notifications(&self, ctx: &Context) -> Result<(), ClientError> {
        use neptunium_http::endpoints::channel::AcknowledgeNewPinNotifications;

        Ok(ctx
            .get_http_client()
            .execute(AcknowledgeNewPinNotifications {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn add_user_to_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
    ) -> Result<(), ClientError> {
        Ok(AddUserToGroupDm {
            channel_id: self.get_channel_id(),
            user_id,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn remove_user_from_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
        silent: bool,
    ) -> Result<(), ClientError> {
        Ok(RemoveUserFromGroupDm {
            channel_id: self.get_channel_id(),
            silent,
            user_id,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn list_rtc_regions(
        &self,
        ctx: &Context,
    ) -> Result<Vec<ListRtcRegionsResponseEntry>, ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(ListRtcRegions {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn indicate_typing(&self, ctx: &Context) -> Result<(), ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(IndicateTyping {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn create_invite(
        &self,
        ctx: &Context,
        options: CreateChannelInviteOptions,
    ) -> Result<Cached<InviteWithMetadata>, ClientError> {
        Ok(CreateChannelInvite {
            channel_id: self.get_channel_id(),
            options,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn list_invites(
        &self,
        ctx: &Context,
    ) -> Result<Vec<Cached<InviteWithMetadata>>, ClientError> {
        Ok(ListChannelInvites {
            channel_id: self.get_channel_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn list_webhooks(&self, ctx: &Context) -> Result<Vec<Webhook>, ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(ListChannelWebhooks {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn create_webhook(
        &self,
        ctx: &Context,
        name: String,
        avatar: Option<String>,
    ) -> Result<Webhook, ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(CreateWebhook {
                channel_id: self.get_channel_id(),
                name,
                avatar,
            })
            .await?)
    }

    async fn pin(&self, ctx: &Context) -> Result<(), ClientError> {
        ctx.get_http_client()
            .execute(PinDirectMessageChannel {
                channel_id: self.get_channel_id(),
            })
            .await?;
        Ok(())
    }

    async fn unpin(&self, ctx: &Context) -> Result<(), ClientError> {
        ctx.get_http_client()
            .execute(UnpinDirectMessageChannel {
                channel_id: self.get_channel_id(),
            })
            .await?;
        Ok(())
    }

    async fn get_slowmode_information(
        &self,
        ctx: &Context,
    ) -> Result<ChannelSlowmodeInformation, ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(GetChannelSlowmodeInformation {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn create_attachments(
        &self,
        ctx: &Context,
        attachments: Vec<CreateAttachmentsInChannelAttachment>,
    ) -> Result<CreateAttachmentsInChannelResponse, ClientError> {
        Ok(ctx
            .get_http_client()
            .execute(CreateAttachmentsInChannel {
                channel_id: self.get_channel_id(),
                attachments,
            })
            .await?)
    }
}

#[async_trait]
pub trait IntoCachedChannel {
    /// Convert this into a `Cached<CachedChannel>`, possibly getting it from the API.
    async fn into_cached_channel(self, ctx: &Context)
    -> Result<Cached<CachedChannel>, ClientError>;
}

#[async_trait]
impl IntoCachedChannel for Channel {
    async fn into_cached_channel(
        self,
        ctx: &Context,
    ) -> Result<Cached<CachedChannel>, ClientError> {
        let cached_channel = CachedChannel::from_channel(self, &ctx.cache);
        Ok(cached_channel.insert_and_return(&ctx.cache))
    }
}

#[async_trait]
impl IntoCachedChannel for CachedChannel {
    async fn into_cached_channel(
        self,
        ctx: &Context,
    ) -> Result<Cached<CachedChannel>, ClientError> {
        Ok(self.insert_and_return(&ctx.cache))
    }
}

#[async_trait]
impl IntoCachedChannel for Cached<CachedChannel> {
    async fn into_cached_channel(
        self,
        _ctx: &Context,
    ) -> Result<Cached<CachedChannel>, ClientError> {
        Ok(self)
    }
}

#[async_trait]
impl IntoCachedChannel for Id<ChannelMarker> {
    async fn into_cached_channel(
        self,
        ctx: &Context,
    ) -> Result<Cached<CachedChannel>, ClientError> {
        self.get(ctx).await
    }
}
