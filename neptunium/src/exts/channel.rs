use async_trait::async_trait;
use neptunium_http::endpoints::channel::{
    delete_channel::DeleteChannel,
    delete_permission_overwrite::DeletePermissionOverwrite,
    fetch_channel::FetchChannel,
    get_call_eligibility_status::{CallEligibilityStatus, GetCallEligibilityStatus},
    messages::{
        bulk_delete_messages::BulkDeleteMessages,
        create_message::{CreateMessage, CreateMessageBody},
        list_channel_messages::{ListChannelMessages, ListChannelMessagesParams},
    },
    ring_call_recipients::RingCallRecipients,
    set_permission_overwrite::{PermissionOverwriteUpdate, SetPermissionOverwrite},
    stop_ringing_call_recipients::StopRingingCallRecipients,
    update_call_region::UpdateCallRegion,
    update_channel_settings::{ChannelSettingsUpdates, UpdateChannelSettings},
};
use neptunium_model::{
    channel::{Channel, VoiceRegion, message::Message},
    id::{
        Id,
        marker::{GenericMarker, MessageMarker, UserMarker},
    },
};

use crate::{
    client::error::Error, events::context::Context, internal::traits::channel::ChannelTrait,
};

#[async_trait]
pub trait ChannelExt {
    async fn delete(&self, ctx: &Context) -> Result<(), Error>;
    async fn delete_silent(&self, ctx: &Context) -> Result<(), Error>;
    // TODO: Maybe make a builder or something around the ChannelSettingsUpdates
    // because it's annoying to create ig
    async fn update_settings(
        &self,
        ctx: &Context,
        settings: ChannelSettingsUpdates,
    ) -> Result<Channel, Error>;
    async fn fetch(&self, ctx: &Context) -> Result<Channel, Error>;
    async fn get_call_eligibility_status(
        &self,
        ctx: &Context,
    ) -> Result<CallEligibilityStatus, Error>;
    /// Update the voice region for an ongoing call.
    async fn update_call_region(&self, ctx: &Context, region: VoiceRegion) -> Result<(), Error>;
    /// Sends ringing notifications to specfied users in a call. If the recipients
    /// are set to `None`, rings all channel members.
    async fn ring_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error>;
    /// Stops ringing notifications for specified users in a call. This allows callers
    /// to stop notifying users who have declined or not responded. Pass `None` for the
    /// recipients to stop ringing everyone.
    async fn stop_ringing_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error>;
    async fn list_messages(
        &self,
        ctx: &Context,
        params: ListChannelMessagesParams,
    ) -> Result<Vec<Message>, Error>;
    async fn bulk_delete_messages(
        &self,
        ctx: &Context,
        messages: Vec<Id<MessageMarker>>,
    ) -> Result<(), Error>;
    /// Same as `create_message`.
    async fn send_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Message, Error>;
    async fn create_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Message, Error>;
    async fn set_permission_overwrite(
        &self,
        ctx: &Context,
        update: PermissionOverwriteUpdate,
    ) -> Result<(), Error>;
    async fn delete_permission_overwrite(
        &self,
        ctx: &Context,
        overwrite_id: Id<GenericMarker>,
    ) -> Result<(), Error>;
}

#[async_trait]
impl<T: ChannelTrait> ChannelExt for T {
    async fn delete(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteChannel::builder()
                    .channel_id(self.get_channel_id())
                    .build(),
            )
            .await?)
    }
    async fn delete_silent(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                DeleteChannel::builder()
                    .channel_id(self.get_channel_id())
                    .silent(true)
                    .build(),
            )
            .await?)
    }
    async fn update_settings(
        &self,
        ctx: &Context,
        settings: ChannelSettingsUpdates,
    ) -> Result<Channel, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                UpdateChannelSettings::builder()
                    .channel_id(self.get_channel_id())
                    .updates(settings)
                    .build(),
            )
            .await?)
    }
    async fn fetch(&self, ctx: &Context) -> Result<Channel, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                FetchChannel::builder()
                    .channel_id(self.get_channel_id())
                    .build(),
            )
            .await?)
    }
    async fn get_call_eligibility_status(
        &self,
        ctx: &Context,
    ) -> Result<CallEligibilityStatus, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                GetCallEligibilityStatus::builder()
                    .channel_id(self.get_channel_id())
                    .build(),
            )
            .await?)
    }
    async fn update_call_region(&self, ctx: &Context, region: VoiceRegion) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                UpdateCallRegion::builder()
                    .channel_id(self.get_channel_id())
                    .region(region)
                    .build(),
            )
            .await?)
    }
    async fn ring_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
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
    ) -> Result<Vec<Message>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                ListChannelMessages::builder()
                    .channel_id(self.get_channel_id())
                    .params(params)
                    .build(),
            )
            .await?)
    }
    async fn bulk_delete_messages(
        &self,
        ctx: &Context,
        messages: Vec<Id<MessageMarker>>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                BulkDeleteMessages::builder()
                    .channel_id(self.get_channel_id())
                    .messages(messages)
                    .build(),
            )
            .await?)
    }
    async fn send_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Message, Error> {
        self.create_message(ctx, message).await
    }
    async fn create_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Message, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                CreateMessage::builder()
                    .channel_id(self.get_channel_id())
                    .message(message)
                    .build(),
            )
            .await?)
    }
    async fn set_permission_overwrite(
        &self,
        ctx: &Context,
        update: PermissionOverwriteUpdate,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(SetPermissionOverwrite {
                channel_id: self.get_channel_id(),
                overwrite: update,
            })
            .await?)
    }
    async fn delete_permission_overwrite(
        &self,
        ctx: &Context,
        overwrite_id: Id<GenericMarker>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(DeletePermissionOverwrite {
                channel_id: self.get_channel_id(),
                overwrite_id,
            })
            .await?)
    }
}
