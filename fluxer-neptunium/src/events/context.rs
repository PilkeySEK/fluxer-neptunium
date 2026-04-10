use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "user_api")]
use neptunium_cache_inmemory::CachedMessage;
use neptunium_cache_inmemory::{CachableEndpoint, Cached, CachedChannel};
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
use neptunium_model::{
    channel::message::nonce::Nonce,
    gateway::payload::{
        incoming::UserPrivateResponse,
        outgoing::{
            GuildSubscriptionRequest, LazyRequest, PresenceUpdateOutgoing, RequestGuildMembers,
        },
    },
    guild::{Guild, member::GuildMember},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
};
use tokio::sync::{
    mpsc::{UnboundedSender, unbounded_channel},
    oneshot,
};

use crate::{
    client::{
        ClientMessage,
        error::{ClientErrorKind, Error},
    },
    exts::ChannelExt,
};

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) tx: UnboundedSender<ClientMessage>,
    pub(crate) cache: Arc<neptunium_cache_inmemory::Cache>,
}

// TODO: Add errors docs
#[expect(clippy::missing_errors_doc)]
impl Context {
    #[must_use]
    pub fn get_http_client(&self) -> &Arc<HttpClient> {
        &self.http_client
    }

    /// Update the presence by sending a gateway request. Due to
    /// how the crate is structured currently, this does not block.
    pub async fn update_presence(&self, data: PresenceUpdateOutgoing) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        if self
            .tx
            .send(ClientMessage::UpdatePresence(data, tx))
            .is_err()
        {
            return Err(Error::new(ClientErrorKind::ClientNotPresent));
        }
        if let Ok(result) = rx.await {
            match result {
                Ok(()) => Ok(()),
                Err(e) => Err(Error::new(ClientErrorKind::NetworkError(e))),
            }
        } else {
            Err(Error::new(ClientErrorKind::ClientNotPresent))
        }
    }

    // TODO: Once caching for guild members and presences exists, cache these things here.
    // Or more likely cache them in the GuildMembersChunk struct thing directly!!
    /// Request guild members from the gateway. Internally, this waits for all chunks to have arrived
    /// and then returns the list of members. For a lower-level version, use `request_guild_members_raw`.
    ///
    /// **Note:** You do not need to set a `nonce` in the request, this function will overwrite it.
    pub async fn request_guild_members(
        &self,
        mut data: RequestGuildMembers,
    ) -> Result<Vec<GuildMember>, Error> {
        // This is to make sure that the request has a proper nonce.
        data.nonce = Some(Nonce::generate().0);
        let (oneshot_tx, oneshot_rx) = oneshot::channel();
        let (tx, mut rx) = unbounded_channel();
        if self
            .tx
            .send(ClientMessage::RequestGuildMembers(
                data,
                oneshot_tx,
                Some(tx),
            ))
            .is_err()
        {
            return Err(Error::new(ClientErrorKind::ClientNotPresent));
        }

        match oneshot_rx.await {
            Ok(Err(e)) => return Err(Error::new(ClientErrorKind::NetworkError(e))),
            Err(_) => return Err(Error::new(ClientErrorKind::ClientNotPresent)),
            Ok(Ok(())) => {}
        }

        let mut members = Vec::new();
        while let Some(chunk) = rx.recv().await {
            for member in chunk.members {
                members.push(member);
            }
            if chunk.chunk_index + 1 == chunk.chunk_count {
                break;
            }
        }
        Ok(members)
    }

    /// Send `RequestGuildMembers` to the gateway without waiting for the result.
    /// You will need to manage receiving the members yourself. This can be useful when you require
    /// lower-level control though.
    pub async fn request_guild_members_raw(&self, data: RequestGuildMembers) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        if self
            .tx
            .send(ClientMessage::RequestGuildMembers(data, tx, None))
            .is_err()
        {
            return Err(Error::new(ClientErrorKind::ClientNotPresent));
        }

        match rx.await {
            Ok(Err(e)) => Err(Error::new(ClientErrorKind::NetworkError(e))),
            Err(_) => Err(Error::new(ClientErrorKind::ClientNotPresent)),
            Ok(Ok(())) => Ok(()),
        }
    }

    pub async fn update_guild_event_subscriptions(
        &self,
        subscriptions: HashMap<Id<GuildMarker>, GuildSubscriptionRequest>,
    ) -> Result<(), Error> {
        let (tx, mut rx) = unbounded_channel();
        if self
            .tx
            .send(ClientMessage::SendLazyRequest(
                LazyRequest { subscriptions },
                tx,
            ))
            .is_err()
        {
            return Err(Error::new(ClientErrorKind::ClientNotPresent));
        }
        if let Some(result) = rx.recv().await {
            match result {
                Ok(()) => Ok(()),
                Err(e) => Err(Error::new(ClientErrorKind::NetworkError(e))),
            }
        } else {
            Err(Error::new(ClientErrorKind::ClientNotPresent))
        }
    }

    /// Fetch a channel from the API.
    /// # Errors
    /// Returns an error if there was a network error, the API did not return OK,
    /// or the API returned unexpected data that could not be parsed.
    pub async fn fetch_channel(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Result<Cached<CachedChannel>, Error> {
        channel_id.get(self).await
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

    pub async fn get_own_profile(&self) -> Result<Cached<UserPrivateResponse>, Error> {
        Ok(GetCurrentUserProfile
            .execute_cached(self.get_http_client(), &self.cache)
            .await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn update_own_profile(
        &self,
        body: UpdateCurrentUserProfile,
    ) -> Result<Cached<UserPrivateResponse>, Error> {
        Ok(body
            .execute_cached(self.get_http_client(), &self.cache)
            .await?)
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
    pub async fn list_own_private_channels(&self) -> Result<Vec<Cached<CachedChannel>>, Error> {
        Ok(ListPrivateChannels
            .execute_cached(self.get_http_client(), &self.cache)
            .await?)
    }

    /// Create a new DM or group DM channel.
    pub async fn create_private_channel(
        &self,
        body: CreatePrivateChannel,
    ) -> Result<Cached<CachedChannel>, Error> {
        Ok(body
            .execute_cached(self.get_http_client(), &self.cache)
            .await?)
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
        // We do not remove the current user from the cache because it is OnceCell<T> and I don't want to handle that currently.
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
    ) -> Result<Vec<Cached<CachedMessage>>, Error> {
        // let messages = self.http_client.execute(params).await?;
        // Ok(self.cache.batch_insert(messages))
        Ok(params
            .execute_cached(self.get_http_client(), &self.cache)
            .await?)
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
    ) -> Result<HashMap<Id<ChannelMarker>, Cached<CachedMessage>>, Error> {
        use neptunium_http::endpoints::channel::PreloadMessagesForChannels;

        Ok(PreloadMessagesForChannels {
            channels: channel_ids,
        }
        .execute_cached(self.get_http_client(), &self.cache)
        .await?)
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
    pub async fn get_settings(&self) -> Result<Cached<UserSettings>, Error> {
        use neptunium_http::endpoints::users::GetUserSettings;

        Ok(GetUserSettings
            .execute_cached(self.get_http_client(), &self.cache)
            .await?)
    }

    #[cfg(feature = "user_api")]
    pub async fn update_settings(
        &self,
        body: UpdateUserSettings,
    ) -> Result<Cached<UserSettings>, Error> {
        Ok(body
            .execute_cached(self.get_http_client(), &self.cache)
            .await?)
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
