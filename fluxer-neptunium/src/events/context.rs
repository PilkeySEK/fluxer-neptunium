#[cfg(feature = "user_api")]
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "user_api")]
use neptunium_http::endpoints::users::{
    CancelBulkMessageDeletionResponse, DisableTotpMfa, EnableTotpMfa,
    GetDataHarvestDownloadUrlResponse, GetMfaBackupCodes, ListCurrentUserMentions,
    MfaBackupCodesResponse, RequestDataHarvestResponse, RequestNewEmailAddress,
    RequestNewEmailAddressResponse, StartEmailChangeResponse, UpdateCurrentUserProfile,
    UpdateDmNotificationSettings, VerifyNewEmailAddress, VerifyNewEmailAddressResponse,
    VerifyOriginalEmailAddress, VerifyOriginalEmailAddressResponse,
};
use neptunium_http::{
    client::HttpClient,
    endpoints::{
        channel::{CreatePrivateChannel, ListPrivateChannels},
        gateway::{GatewayInformation, GetGatewayInformation},
        guild::{ListCurrentUserGuilds, ListCurrentUserGuildsParams},
        users::GetCurrentUserProfile,
    },
};
use neptunium_model::{
    channel::Channel,
    gateway::payload::{incoming::UserPrivateResponse, outgoing::PresenceUpdateOutgoing},
    guild::Guild,
    id::{Id, marker::ChannelMarker},
};
#[cfg(feature = "user_api")]
use neptunium_model::{
    channel::message::Message,
    user::{
        auth::SudoVerification, data_harvest::DataHarvestResponse, gifts::GiftPrivateResponse,
        settings::UserGuildSettings,
    },
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    client::{ClientMessage, error::Error},
    exts::ChannelExt,
};

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) tx: UnboundedSender<ClientMessage>,
}

impl Context {
    #[must_use]
    pub fn get_http_client(&self) -> &Arc<HttpClient> {
        &self.http_client
    }

    // TODO: Make this be async and block
    /// Update the presence by sending a gateway request. Due to
    /// how the crate is structured currently, this does not block.
    pub fn update_presence(&self, data: PresenceUpdateOutgoing) {
        // ignoring potential error caused by the channel being closed
        // TODO: Maybe not ignore it
        let _ = self.tx.send(ClientMessage::UpdatePresence(data));
    }

    /// Fetch a channel from the API.
    /// # Errors
    /// Returns an error if there was a network error, the API did not return OK,
    /// or the API returned unexpected data that could not be parsed.
    pub async fn fetch_channel(&self, channel_id: Id<ChannelMarker>) -> Result<Channel, Error> {
        channel_id.fetch(self).await
    }

    /// Get the gateway information from the API.
    /// # Errors
    /// Returns an error if there was a network error, the API did not return OK,
    /// or the API returned unexpected data that could not be parsed.
    pub async fn get_gateway_information(&self) -> Result<GatewayInformation, Error> {
        Ok(self.http_client.execute(GetGatewayInformation).await?)
    }

    pub async fn list_own_guilds(&self) -> Result<Vec<Guild>, Error> {
        Ok(self
            .http_client
            .execute(
                ListCurrentUserGuilds::builder()
                    .params(ListCurrentUserGuildsParams::builder().build())
                    .build(),
            )
            .await?)
    }

    pub async fn list_own_guilds_with_params(
        &self,
        params: ListCurrentUserGuildsParams,
    ) -> Result<Vec<Guild>, Error> {
        Ok(self
            .http_client
            .execute(ListCurrentUserGuilds::builder().params(params).build())
            .await?)
    }

    pub async fn get_own_profile(&self) -> Result<UserPrivateResponse, Error> {
        Ok(self.http_client.execute(GetCurrentUserProfile).await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn update_own_profile(
        &self,
        body: UpdateCurrentUserProfile,
    ) -> Result<UserPrivateResponse, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn forget_authorized_ips_for_current_user(
        &self,
        auth: SudoVerification,
    ) -> Result<(), Error> {
        use neptunium_http::endpoints::users::ForgetAuthorizedIps;

        Ok(self
            .http_client
            .execute(ForgetAuthorizedIps { auth })
            .await?)
    }

    /// List DM channels. This includes group DMs.
    pub async fn list_own_private_channels(&self) -> Result<Vec<Channel>, Error> {
        Ok(self.http_client.execute(ListPrivateChannels).await?)
    }

    /// Create a new DM or group DM channel.
    pub async fn create_private_channel(
        &self,
        body: CreatePrivateChannel,
    ) -> Result<Channel, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Alternative endpoint to preload and cache messages for multiple channels to improve performance when opening those channels.
    #[cfg(feature = "user_api")]
    pub async fn preload_messages_for_channels(
        &self,
        channel_ids: Vec<Id<ChannelMarker>>,
    ) -> Result<HashMap<Id<ChannelMarker>, Message>, Error> {
        use neptunium_http::endpoints::channel::PreloadMessagesForChannels;

        Ok(self
            .http_client
            .execute(PreloadMessagesForChannels {
                channels: channel_ids,
            })
            .await?)
    }

    /// Permanently deletes the current user’s account and all associated data.
    /// This action is irreversible and will remove all user data, messages, and connections.
    #[cfg(feature = "user_api")]
    pub async fn delete_own_account(&self, auth: SudoVerification) -> Result<(), Error> {
        use neptunium_http::endpoints::users::DeleteCurrentUserAccount;

        Ok(self
            .http_client
            .execute(DeleteCurrentUserAccount { auth })
            .await?)
    }

    /// Temporarily disables the current user’s account. The account can be re-enabled by logging in again.
    /// User data is preserved but the account will be inaccessible during the disabled period.
    #[cfg(feature = "user_api")]
    pub async fn disable_own_account(&self, auth: SudoVerification) -> Result<(), Error> {
        use neptunium_http::endpoints::users::DisableCurrentUserAccount;

        Ok(self
            .http_client
            .execute(DisableCurrentUserAccount { auth })
            .await?)
    }

    /// Starts a dedicated bounced-email recovery flow.
    /// Sends a verification code to the replacement email without requiring
    /// verification of the old bounced email address.
    #[cfg(feature = "user_api")]
    pub async fn request_replacement_email_for_bounced_address(
        &self,
        new_email: String,
    ) -> Result<RequestNewEmailAddressResponse, Error> {
        use neptunium_http::endpoints::users::RequestReplacementEmailForBouncedAddress;

        Ok(self
            .http_client
            .execute(RequestReplacementEmailForBouncedAddress { new_email })
            .await?)
    }

    /// Resends the verification code for the bounced-email recovery flow to the replacement email address.
    #[cfg(feature = "user_api")]
    pub async fn resend_replacement_email_code(&self, ticket: String) -> Result<(), Error> {
        use neptunium_http::endpoints::users::ResendReplacementEmailCode;

        Ok(self
            .http_client
            .execute(ResendReplacementEmailCode { ticket })
            .await?)
    }

    /// Completes bounced-email recovery by verifying the replacement email code,
    /// updating the account email, and clearing email-related suspicious-activity requirements.
    #[cfg(feature = "user_api")]
    pub async fn verify_replacement_email_for_bounced_address(
        &self,
        ticket: String,
        code: String,
    ) -> Result<UserPrivateResponse, Error> {
        use neptunium_http::endpoints::users::VerifyReplacementEmailForBouncedAddress;

        Ok(self
            .http_client
            .execute(VerifyReplacementEmailForBouncedAddress { ticket, code })
            .await?)
    }

    /// Requests to change email to a new address. Requires proof of original email verification.
    /// Sends confirmation code to new email address for verification.
    #[cfg(feature = "user_api")]
    pub async fn request_new_email_address(
        &self,
        body: RequestNewEmailAddress,
    ) -> Result<RequestNewEmailAddressResponse, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Resends a confirmation code to the new email address during the email change process.
    /// Use this if the new email confirmation was not received. Requires valid email change ticket.
    #[cfg(feature = "user_api")]
    pub async fn resend_new_email_confirmation(&self, ticket: String) -> Result<(), Error> {
        use neptunium_http::endpoints::users::ResendNewEmailConfirmation;

        Ok(self
            .http_client
            .execute(ResendNewEmailConfirmation { ticket })
            .await?)
    }

    /// Resends a confirmation code to the user’s original email address during the email change process.
    /// Use this if the original confirmation email was not received. Requires valid email change ticket.
    #[cfg(feature = "user_api")]
    pub async fn resend_original_email_confirmation(&self, ticket: String) -> Result<(), Error> {
        use neptunium_http::endpoints::users::ResendOriginalEmailConfirmation;

        Ok(self
            .http_client
            .execute(ResendOriginalEmailConfirmation { ticket })
            .await?)
    }

    /// Initiates an email change process. Generates a ticket for verifying
    /// the original email address before requesting a new email.
    /// Returns ticket for use in subsequent email change steps.
    #[cfg(feature = "user_api")]
    pub async fn start_email_change(&self) -> Result<StartEmailChangeResponse, Error> {
        use neptunium_http::endpoints::users::StartEmailChange;

        Ok(self.http_client.execute(StartEmailChange).await?)
    }

    /// Completes the email change process by verifying the new email address with a confirmation code.
    /// Returns an email token that confirms the email change. After this step, the user may need to re-authenticate.
    #[cfg(feature = "user_api")]
    pub async fn verify_new_email_address(
        &self,
        body: VerifyNewEmailAddress,
    ) -> Result<VerifyNewEmailAddressResponse, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Verifies ownership of the original email address by validating a confirmation code sent to that address.
    /// Must be completed before requesting a new email address. Returns proof token for use in new email request.
    #[cfg(feature = "user_api")]
    pub async fn verify_original_email_address(
        &self,
        body: VerifyOriginalEmailAddress,
    ) -> Result<VerifyOriginalEmailAddressResponse, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Lists all gift codes created by the authenticated user.
    #[cfg(feature = "user_api")]
    pub async fn list_gifts(&self) -> Result<Vec<GiftPrivateResponse>, Error> {
        use neptunium_http::endpoints::users::ListUserGifts;

        Ok(self.http_client.execute(ListUserGifts).await?)
    }

    /// Updates the user’s notification settings for direct messages and group DMs.
    /// Controls how DM notifications are handled.
    ///
    /// This returns `UserGuildSettings` because the Fluxer backend treats the
    /// DMs as a guild in this case. (So, a guild with many channels, a channel
    /// for each (group-) DM the user has.)
    #[cfg(feature = "user_api")]
    pub async fn update_dm_notification_settings(
        &self,
        body: UpdateDmNotificationSettings,
    ) -> Result<UserGuildSettings, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Requests a data harvest of all user data and content. Initiates an asynchronous process
    /// to compile and prepare all data for download in a portable format.
    #[cfg(feature = "user_api")]
    pub async fn request_data_harvest(&self) -> Result<RequestDataHarvestResponse, Error> {
        use neptunium_http::endpoints::users::RequestDataHarvest;

        Ok(self.http_client.execute(RequestDataHarvest).await?)
    }

    /// Retrieves the status of the most recent data harvest request.
    /// Returns `None` if no harvest has been requested yet.
    #[cfg(feature = "user_api")]
    pub async fn get_latest_data_harvest(&self) -> Result<Option<DataHarvestResponse>, Error> {
        use neptunium_http::endpoints::users::GetLatestDataHarvest;

        Ok(self.http_client.execute(GetLatestDataHarvest).await?)
    }

    /// Retrieves detailed status information for a specific data harvest.
    #[cfg(feature = "user_api")]
    pub async fn get_data_harvest_status(
        &self,
        harvest_id: String,
    ) -> Result<DataHarvestResponse, Error> {
        use neptunium_http::endpoints::users::GetDataHarvestStatus;

        Ok(self
            .http_client
            .execute(GetDataHarvestStatus { harvest_id })
            .await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn get_data_harvest_download_url(
        &self,
        harvest_id: String,
    ) -> Result<GetDataHarvestDownloadUrlResponse, Error> {
        use neptunium_http::endpoints::users::GetDataHarvestDownloadUrl;

        Ok(self
            .http_client
            .execute(GetDataHarvestDownloadUrl { harvest_id })
            .await?)
    }

    /// Retrieves messages where the current user was mentioned.
    #[cfg(feature = "user_api")]
    pub async fn list_own_mentions(
        &self,
        params: ListCurrentUserMentions,
    ) -> Result<Vec<Message>, Error> {
        Ok(self.http_client.execute(params).await?)
    }

    /// Initiates bulk deletion of all messages sent by the current user.
    /// The deletion process is asynchronous and may take time to complete. User data remains intact.
    #[cfg(feature = "user_api")]
    pub async fn request_bulk_message_deletion(&self, auth: SudoVerification) -> Result<(), Error> {
        use neptunium_http::endpoints::users::RequestBulkMessageDeletion;

        Ok(self
            .http_client
            .execute(RequestBulkMessageDeletion { auth })
            .await?)
    }

    /// Cancels an in-progress bulk message deletion request. Can only be used if the deletion has not yet completed.
    #[cfg(feature = "user_api")]
    pub async fn cancel_bulk_message_deletion(
        &self,
    ) -> Result<CancelBulkMessageDeletionResponse, Error> {
        use neptunium_http::endpoints::users::CancelBulkMessageDeletion;

        Ok(self.http_client.execute(CancelBulkMessageDeletion).await?)
    }

    /// Staff-only endpoint for testing bulk message deletion functionality.
    /// Creates a test deletion request with a 1-minute delay.
    #[cfg(feature = "staff_api")]
    pub async fn test_bulk_message_deletion(&self) -> Result<(), Error> {
        use neptunium_http::endpoints::users::TestBulkMessageDeletion;

        Ok(self.http_client.execute(TestBulkMessageDeletion).await?)
    }

    /// Generate and retrieve new backup codes for account recovery. Old codes are invalidated.
    #[cfg(feature = "user_api")]
    pub async fn get_mfa_backup_codes(
        &self,
        body: GetMfaBackupCodes,
    ) -> Result<MfaBackupCodesResponse, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Disable SMS-based multi-factor authentication on the current account.
    #[cfg(feature = "user_api")]
    pub async fn disable_sms_mfa(&self, auth: SudoVerification) -> Result<(), Error> {
        use neptunium_http::endpoints::users::DisableSmsMfa;

        Ok(self.http_client.execute(DisableSmsMfa { auth }).await?)
    }

    /// Enable SMS-based multi-factor authentication on the current account.
    /// Requires a verified phone number.
    #[cfg(feature = "user_api")]
    pub async fn enable_sms_mfa(&self, auth: SudoVerification) -> Result<(), Error> {
        use neptunium_http::endpoints::users::EnableSmsMfa;

        Ok(self.http_client.execute(EnableSmsMfa { auth }).await?)
    }

    /// Disable TOTP multi-factor authentication on the current account.
    #[cfg(feature = "user_api")]
    pub async fn disable_totp_mfa(&self, body: DisableTotpMfa) -> Result<(), Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Enable time-based one-time password (TOTP) MFA on the current account.
    #[cfg(feature = "user_api")]
    pub async fn enable_totp_mfa(
        &self,
        body: EnableTotpMfa,
    ) -> Result<MfaBackupCodesResponse, Error> {
        Ok(self.http_client.execute(body).await?)
    }
}
