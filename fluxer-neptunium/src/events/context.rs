#[cfg(feature = "user_api")]
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "user_api")]
use neptunium_http::endpoints::{
    channel::ScheduledMessageResponse,
    users::{
        AddPhoneNumberToAccount, CompletePasswordChange, DeleteWebauthnCredential, DisableTotpMfa,
        EnableTotpMfa, GetDataHarvestDownloadUrlResponse, GetMfaBackupCodes,
        GetSudoWebauthnAuthenticationOptionsResponse, GetWebauthnRegistrationOptionsResponse,
        ListCurrentUserMentions, ListPushSubscriptionsResponseEntry,
        ListSudoMfaAuthenticationMethodsResponse, ListWebauthnCredentialsResponseEntry,
        MfaBackupCodesResponse, RegisterWebauthnCredential, RequestDataHarvestResponse,
        RequestNewEmailAddress, RequestNewEmailAddressResponse, StartEmailChangeResponse,
        StartPasswordChangeResponse, SubscribeToPushNotifications,
        SubscribeToPushNotificationsResponse, UpdateCurrentUserProfile,
        UpdateDmNotificationSettings, UpdateRelationship, UpdateUserSettings,
        UpdateWebauthnCredential, VerifyNewEmailAddress, VerifyNewEmailAddressResponse,
        VerifyOriginalEmailAddress, VerifyOriginalEmailAddressResponse, VerifyPasswordChangeCode,
        VerifyPasswordChangeCodeResponse, VerifyPhoneCode, VerifyPhoneCodeResponse,
    },
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
    id::marker::UserMarker,
    user::{
        auth::SudoVerification,
        data_harvest::DataHarvestResponse,
        gifts::GiftPrivateResponse,
        relationship::Relationship,
        saved_messages::SavedMessage,
        settings::{UserGuildSettings, UserSettings},
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
    pub(crate) cache: crate::cache::Cache,
}

// TODO: Add errors docs
#[expect(clippy::missing_errors_doc)]
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

    pub async fn get_own_profile(&self) -> Result<Arc<UserPrivateResponse>, Error> {
        let cached_current_user = Arc::clone(&self.cache.current_user);
        let cached_current_user = cached_current_user.read_owned().await.clone();
        if let Some(cached_current_user) = cached_current_user {
            return Ok(cached_current_user);
        }
        let current_user = self.http_client.execute(GetCurrentUserProfile).await?;
        let current_user = Arc::new(current_user);
        let mut guard = Arc::clone(&self.cache.current_user).write_owned().await;
        *guard = Some(Arc::clone(&current_user));
        Ok(current_user)
    }

    #[cfg(feature = "user_api")]
    pub async fn update_own_profile(
        &self,
        body: UpdateCurrentUserProfile,
    ) -> Result<Arc<UserPrivateResponse>, Error> {
        let new_current_user = self.http_client.execute(body).await?;
        let new_current_user = Arc::new(new_current_user);
        *Arc::clone(&self.cache.current_user).write_owned().await =
            Some(Arc::clone(&new_current_user));
        Ok(new_current_user)
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
    pub async fn list_own_private_channels(&self) -> Result<Vec<Arc<Channel>>, Error> {
        let channels = self.http_client.execute(ListPrivateChannels).await?;
        Ok(self.cache.batch_insert(channels))
    }

    /// Create a new DM or group DM channel.
    pub async fn create_private_channel(
        &self,
        body: CreatePrivateChannel,
    ) -> Result<Arc<Channel>, Error> {
        let channel = self.http_client.execute(body).await?;
        Ok(self.cache.insert(channel))
    }

    /// Alternative endpoint to preload and cache messages for multiple channels to improve performance when opening those channels.
    #[cfg(feature = "user_api")]
    pub async fn preload_messages_for_channels_alternative(
        &self,
        channel_ids: Vec<Id<ChannelMarker>>,
    ) -> Result<HashMap<Id<ChannelMarker>, Message>, Error> {
        use neptunium_http::endpoints::channel::PreloadMessagesForChannelsAlternative;

        Ok(self
            .http_client
            .execute(PreloadMessagesForChannelsAlternative {
                channels: channel_ids,
            })
            .await?)
    }

    /// Permanently deletes the current user’s account and all associated data.
    /// This action is irreversible and will remove all user data, messages, and connections.
    #[cfg(feature = "user_api")]
    pub async fn delete_own_account(&self, auth: SudoVerification) -> Result<(), Error> {
        use neptunium_http::endpoints::users::DeleteCurrentUserAccount;

        self.http_client
            .execute(DeleteCurrentUserAccount { auth })
            .await?;
        // Remove current user from cache
        *self.cache.current_user.clone().write_owned().await = None;
        Ok(())
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
        // No cache update needed, the UserPrivateResponse doesn't have a `disabled` field
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
    ) -> Result<Vec<Arc<Message>>, Error> {
        let messages = self.http_client.execute(params).await?;
        Ok(self.cache.batch_insert(messages))
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
    pub async fn cancel_bulk_message_deletion(&self) -> Result<(), Error> {
        use neptunium_http::endpoints::users::CancelBulkMessageDeletion;

        self.http_client.execute(CancelBulkMessageDeletion).await?;
        Ok(())
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

    /// Retrieve all registered WebAuthn credentials (security keys, biometric devices) for the current user.
    #[expect(clippy::doc_markdown)]
    #[cfg(feature = "user_api")]
    pub async fn list_webauthn_credentials(
        &self,
    ) -> Result<Vec<ListWebauthnCredentialsResponseEntry>, Error> {
        use neptunium_http::endpoints::users::ListWebauthnCredentials;

        Ok(self.http_client.execute(ListWebauthnCredentials).await?)
    }

    /// Complete registration of a new WebAuthn credential (security key or biometric device).
    #[expect(clippy::doc_markdown)]
    #[cfg(feature = "user_api")]
    pub async fn register_webauthn_credential(
        &self,
        body: RegisterWebauthnCredential,
    ) -> Result<(), Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Generate challenge and options to register a new WebAuthn credential.
    #[expect(clippy::doc_markdown)]
    #[cfg(feature = "user_api")]
    pub async fn get_webauthn_registration_options(
        &self,
        auth: SudoVerification,
    ) -> Result<GetWebauthnRegistrationOptionsResponse, Error> {
        use neptunium_http::endpoints::users::GetWebauthnRegistrationOptions;

        Ok(self
            .http_client
            .execute(GetWebauthnRegistrationOptions { auth })
            .await?)
    }

    /// Remove a registered WebAuthn credential from the current account.
    #[expect(clippy::doc_markdown)]
    #[cfg(feature = "user_api")]
    pub async fn delete_webauthn_credential(
        &self,
        body: DeleteWebauthnCredential,
    ) -> Result<(), Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Update the name or settings of a registered WebAuthn credential.
    #[expect(clippy::doc_markdown)]
    #[cfg(feature = "user_api")]
    pub async fn update_webauthn_credential(
        &self,
        body: UpdateWebauthnCredential,
    ) -> Result<(), Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Retrieves all notes the current user has written about other users.
    #[cfg(feature = "user_api")]
    pub async fn list_user_notes(&self) -> Result<HashMap<Id<UserMarker>, String>, Error> {
        use neptunium_http::endpoints::users::ListCurrentUserNotes;

        Ok(self.http_client.execute(ListCurrentUserNotes).await?)
    }

    /// Completes the password change after email verification. Invalidates all existing sessions.
    #[cfg(feature = "user_api")]
    pub async fn complete_password_change(
        &self,
        body: CompletePasswordChange,
    ) -> Result<(), Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Resends the verification code for a password change. Use if the original code was not received.
    #[cfg(feature = "user_api")]
    pub async fn resend_password_change_verification_code(
        &self,
        ticket: String,
    ) -> Result<(), Error> {
        use neptunium_http::endpoints::users::ResendPasswordChangeVerificationCode;

        Ok(self
            .http_client
            .execute(ResendPasswordChangeVerificationCode { ticket })
            .await?)
    }

    /// Initiates a password change process. Sends a verification code to the user’s email address.
    #[cfg(feature = "user_api")]
    pub async fn start_password_change(&self) -> Result<StartPasswordChangeResponse, Error> {
        use neptunium_http::endpoints::users::StartPasswordChange;

        Ok(self.http_client.execute(StartPasswordChange).await?)
    }

    /// Verifies the email code sent during password change.
    #[cfg(feature = "user_api")]
    pub async fn verify_password_change_code(
        &self,
        body: VerifyPasswordChangeCode,
    ) -> Result<VerifyPasswordChangeCodeResponse, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Add or update the phone number associated with the current account. Phone must be verified before use.
    #[cfg(feature = "user_api")]
    pub async fn add_phone_number_to_account(
        &self,
        body: AddPhoneNumberToAccount,
    ) -> Result<(), Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Remove the phone number from the current account. SMS MFA will be disabled if enabled.
    #[cfg(feature = "user_api")]
    pub async fn remove_phone_number_from_account(
        &self,
        auth: SudoVerification,
    ) -> Result<(), Error> {
        use neptunium_http::endpoints::users::RemovePhoneNumberFromAccount;

        Ok(self
            .http_client
            .execute(RemovePhoneNumberFromAccount { auth })
            .await?)
    }

    /// Request a verification code to be sent via SMS to the provided phone number.
    #[cfg(feature = "user_api")]
    pub async fn send_phone_verification_code(&self, phone: String) -> Result<(), Error> {
        use neptunium_http::endpoints::users::SendPhoneVerificationCode;

        Ok(self
            .http_client
            .execute(SendPhoneVerificationCode { phone })
            .await?)
    }

    /// Verify a phone number by confirming the SMS verification code.
    #[cfg(feature = "user_api")]
    pub async fn verify_phone_code(
        &self,
        body: VerifyPhoneCode,
    ) -> Result<VerifyPhoneCodeResponse, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Preloads and caches messages for multiple channels to improve performance when opening those channels.
    #[cfg(feature = "user_api")]
    pub async fn preload_messages_for_channels(
        &self,
        channel_ids: Vec<Id<ChannelMarker>>,
    ) -> Result<HashMap<Id<ChannelMarker>, Arc<Message>>, Error> {
        use neptunium_http::endpoints::channel::PreloadMessagesForChannels;

        let messages = self
            .http_client
            .execute(PreloadMessagesForChannels {
                channels: channel_ids,
            })
            .await?;
        Ok(messages
            .into_iter()
            .map(|(channel_id, message)| (channel_id, self.cache.insert(message)))
            .collect())
    }

    /// Staff-only endpoint that clears premium status and related premium metadata for the current user account.
    #[cfg(feature = "staff_api")]
    pub async fn reset_own_premium_state(&self) -> Result<(), Error> {
        use neptunium_http::endpoints::users::ResetCurrentUserPremiumState;

        Ok(self
            .http_client
            .execute(ResetCurrentUserPremiumState)
            .await?)
    }

    /// Registers a new push notification subscription for the current user.
    /// Takes push endpoint and encryption keys from a Web Push API subscription.
    #[cfg(feature = "user_api")]
    pub async fn subscribe_to_push_notifications(
        &self,
        body: SubscribeToPushNotifications,
    ) -> Result<SubscribeToPushNotificationsResponse, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    /// Retrieves all push notification subscriptions for the current user.
    #[cfg(feature = "user_api")]
    pub async fn list_push_subscriptions(
        &self,
    ) -> Result<Vec<ListPushSubscriptionsResponseEntry>, Error> {
        use neptunium_http::endpoints::users::ListPushSubscriptions;

        let response = self.http_client.execute(ListPushSubscriptions).await?;

        Ok(response.subscriptions)
    }

    /// Unregisters a push notification subscription for the current user.
    /// Push notifications will no longer be sent to this subscription endpoint.
    #[cfg(feature = "user_api")]
    pub async fn unsubscribe_from_push_notifications(
        &self,
        subscription_id: String,
    ) -> Result<(), Error> {
        use neptunium_http::endpoints::users::UnsubscribeFromPushNotifications;

        self.http_client
            .execute(UnsubscribeFromPushNotifications { subscription_id })
            .await?;
        Ok(())
    }

    /// Retrieves all relationships for the current user, including friends, friend requests (incoming and outgoing), and blocked users.
    #[cfg(feature = "user_api")]
    pub async fn list_relationships(&self) -> Result<Vec<Relationship>, Error> {
        use neptunium_http::endpoints::users::ListRelationships;

        Ok(self.http_client.execute(ListRelationships).await?)
    }

    /// Sends a friend request to a user identified by username tag (username#discriminator).
    #[cfg(feature = "user_api")]
    pub async fn send_friend_request_by_tag(
        &self,
        username: String,
        discriminator: String,
    ) -> Result<Relationship, Error> {
        use neptunium_http::endpoints::users::SendFriendRequestByTag;

        Ok(self
            .http_client
            .execute(SendFriendRequestByTag {
                username,
                discriminator,
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn update_relationship(
        &self,
        body: UpdateRelationship,
    ) -> Result<Relationship, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn list_saved_messages(&self, limit: Option<u8>) -> Result<Vec<SavedMessage>, Error> {
        use neptunium_http::endpoints::channel::ListSavedMessages;

        Ok(self
            .http_client
            .execute(ListSavedMessages { limit })
            .await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn list_scheduled_message(&self) -> Result<Vec<ScheduledMessageResponse>, Error> {
        use neptunium_http::endpoints::channel::ListScheduledMessages;

        Ok(self.http_client.execute(ListScheduledMessages).await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn get_settings(&self) -> Result<UserSettings, Error> {
        use neptunium_http::endpoints::users::GetUserSettings;

        Ok(self.http_client.execute(GetUserSettings).await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn update_settings(&self, body: UpdateUserSettings) -> Result<UserSettings, Error> {
        Ok(self.http_client.execute(body).await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn list_sudo_mfa_authentication_methods(
        &self,
    ) -> Result<ListSudoMfaAuthenticationMethodsResponse, Error> {
        use neptunium_http::endpoints::users::ListSudoMfaAuthenticationMethods;

        Ok(self
            .http_client
            .execute(ListSudoMfaAuthenticationMethods)
            .await?)
    }

    /// Request an SMS code to be sent for sudo mode verification.
    #[cfg(feature = "user_api")]
    pub async fn send_sudo_sms_code(&self) -> Result<(), Error> {
        use neptunium_http::endpoints::users::SendSudoSmsCode;

        Ok(self.http_client.execute(SendSudoSmsCode).await?)
    }

    /// Generate WebAuthn challenge for sudo mode verification using a registered security key or biometric device.
    #[expect(clippy::doc_markdown)]
    #[cfg(feature = "user_api")]
    pub async fn get_sudo_webauthn_authentication_options(
        &self,
    ) -> Result<GetSudoWebauthnAuthenticationOptionsResponse, Error> {
        use neptunium_http::endpoints::users::GetSudoWebauthnAuthenticationOptions;

        Ok(self
            .http_client
            .execute(GetSudoWebauthnAuthenticationOptions)
            .await?)
    }

    /// Checks if a username and discriminator combination is available.
    /// Returns `true` if the tag is available, and `false` if it is taken by another user.
    #[cfg(feature = "user_api")]
    pub async fn check_username_tag_availability(
        &self,
        username: String,
        discriminator: String,
    ) -> Result<bool, Error> {
        use neptunium_http::endpoints::users::CheckUsernameTagAvailability;

        let response = self
            .http_client
            .execute(CheckUsernameTagAvailability {
                username,
                discriminator,
            })
            .await?;

        Ok(!response.taken)
    }
}
