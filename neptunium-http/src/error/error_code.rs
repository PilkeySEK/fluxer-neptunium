use serde::Deserialize;

#[expect(clippy::doc_markdown)]
#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiErrorCode {
    /// Access to this resource is denied.
    AccessDenied,
    /// This account has been disabled.
    AccountDisabled,
    /// This account is scheduled for deletion.
    AccountScheduledForDeletion,
    /// This account has been permanently suspended.
    AccountSuspendedPermanently,
    /// This account has been temporarily suspended.
    AccountSuspendedTemporarily,
    /// Suspicious activity detected on this account.
    AccountSuspiciousActivity,
    /// Account is too new to join this guild.
    AccountTooNewForGuild,
    /// Access control list must not be empty.
    AclsMustBeNonEmpty,
    /// Admin API key was not found.
    AdminApiKeyNotFound,
    /// You are already friends with this user.
    AlreadyFriends,
    /// Audit log is currently being indexed.
    AuditLogIndexing,
    /// Bots cannot send friend requests.
    BotsCannotSendFriendRequests,
    /// Bot users cannot access this authentication endpoint.
    BotUserAuthEndpointAccessDenied,
    /// Bot users cannot create authentication sessions.
    BotUserAuthSessionCreationDenied,
    /// A call already exists in this channel.
    CallAlreadyExists,
    /// You cannot edit another user message.
    CannotEditOtherUserMessage,
    /// This action cannot be executed on a DM channel.
    CannotExecuteOnDm,
    /// System webhooks cannot be modified.
    CannotModifySystemWebhook,
    /// Cannot modify voice state.
    CannotModifyVoiceState,
    /// Cannot redeem plutonium while having Visionary subscription.
    CannotRedeemPlutoniumWithVisionary,
    /// You cannot report your own guild.
    CannotReportOwnGuild,
    /// You cannot report your own message.
    CannotReportOwnMessage,
    /// You cannot report yourself.
    CannotReportYourself,
    /// Cannot send an empty message.
    CannotSendEmptyMessage,
    /// Cannot send friend request to a blocked user.
    CannotSendFriendRequestToBlockedUser,
    /// Cannot send friend request to yourself.
    CannotSendFriendRequestToSelf,
    /// Cannot send messages in a non-text channel.
    CannotSendMessagesInNonTextChannel,
    /// Cannot send messages to this user.
    CannotSendMessagesToUser,
    /// Cannot shrink reserved slots.
    CannotShrinkReservedSlots,
    /// Captcha verification is required.
    CaptchaRequired,
    /// Channel is currently being indexed.
    ChannelIndexing,
    /// You are timed out in this guild.
    CommunicationDisabled,
    /// Content was blocked.
    ContentBlocked,
    /// Resource creation failed.
    CreationFailed,
    /// Resource deletion failed.
    DeletionFailed,
    /// A discriminator is required.
    DiscriminatorRequired,
    /// Email service is not in testable mode.
    EmailServiceNotTestable,
    /// Email verification is required.
    EmailVerificationRequired,
    /// Explicit content cannot be sent in this channel.
    ExplicitContentCannotBeSent,
    /// This feature is temporarily disabled.
    FeatureTemporarilyDisabled,
    /// File size exceeds the maximum allowed.
    FileSizeTooLarge,
    /// Friend request was blocked.
    FriendRequestBlocked,
    /// A general error occurred.
    GeneralError,
    /// This gift code has already been redeemed.
    GiftCodeAlreadyRedeemed,
    /// Phone verification is required to join this guild.
    GuildPhoneVerificationRequired,
    /// Account verification is required to interact in this guild.
    GuildVerificationRequired,
    /// Handoff code has expired.
    HandoffCodeExpired,
    /// Data harvest request has expired.
    HarvestExpired,
    /// Data harvest failed.
    HarvestFailed,
    /// Data harvest is not yet ready.
    HarvestNotReady,
    /// Data harvest is on cooldown.
    HarvestOnCooldown,
    /// HTTP GET authorize is not supported.
    HttpGetAuthorizeNotSupported,
    /// Instance version mismatch.
    InstanceVersionMismatch,
    /// Invalid access control list format.
    InvalidAclsFormat,
    /// Invalid API origin.
    InvalidApiOrigin,
    /// Invalid authentication token.
    InvalidAuthToken,
    /// Invalid bot flag.
    InvalidBotFlag,
    /// Invalid captcha response.
    InvalidCaptcha,
    /// Invalid channel type for call.
    InvalidChannelTypeForCall,
    /// Invalid channel type.
    InvalidChannelType,
    /// Invalid OAuth2 client.
    InvalidClient,
    /// Invalid client secret.
    InvalidClientSecret,
    /// Invalid DSA report target.
    InvalidDsaReportTarget,
    /// Invalid DSA ticket.
    InvalidDsaTicket,
    /// Invalid DSA verification code.
    InvalidDsaVerificationCode,
    /// Invalid flags format.
    InvalidFlagsFormat,
    /// Invalid request body format.
    InvalidFormBody,
    /// Invalid OAuth2 grant.
    InvalidGrant,
    /// Invalid handoff code.
    InvalidHandoffCode,
    /// Invalid pack type.
    InvalidPackType,
    /// Invalid permissions value.
    InvalidPermissionsInteger,
    /// Permissions value cannot be negative.
    InvalidPermissionsNegative,
    /// Invalid phone number.
    InvalidPhoneNumber,
    /// Invalid phone verification code.
    InvalidPhoneVerificationCode,
    /// Invalid redirect URI.
    InvalidRedirectUri,
    /// Invalid request.
    InvalidRequest,
    /// Invalid response type for non-bot application.
    InvalidResponseTypeForNonBot,
    /// Invalid OAuth2 scope.
    InvalidScope,
    /// Invalid stream key format.
    InvalidStreamKeyFormat,
    /// Invalid stream thumbnail payload.
    InvalidStreamThumbnailPayload,
    /// Invalid sudo token.
    InvalidSudoToken,
    /// Invalid suspicious flags format.
    InvalidSuspiciousFlagsFormat,
    /// Invalid system flag.
    InvalidSystemFlag,
    /// Invalid timestamp.
    InvalidTimestamp,
    /// Invalid token.
    InvalidToken,
    /// Invalid WebAuthn authentication counter.
    InvalidWebauthnAuthenticationCounter,
    /// Invalid WebAuthn credential counter.
    InvalidWebauthnCredentialCounter,
    /// Invalid WebAuthn credential.
    InvalidWebauthnCredential,
    /// Invalid WebAuthn public key format.
    InvalidWebauthnPublicKeyFormat,
    /// Invites are disabled for this guild.
    InvitesDisabled,
    /// IP address authorization is required.
    IpAuthorizationRequired,
    /// IP authorization email resend is on cooldown.
    IpAuthorizationResendCooldown,
    /// IP authorization email resend limit exceeded.
    IpAuthorizationResendLimitExceeded,
    /// This IP address has been banned.
    IpBanned,
    /// Maximum animated emojis limit reached.
    MaxAnimatedEmojis,
    /// Maximum beta codes limit reached.
    MaxBetaCodesReached,
    /// Maximum bookmarks limit reached.
    MaxBookmarks,
    /// Maximum category channels limit reached.
    MaxCategoryChannels,
    /// Maximum emojis limit reached.
    MaxEmojis,
    /// Maximum saved media limit reached.
    #[serde(rename = "MAX_FAVORITE_MEMES")]
    MaxSavedMedia,
    /// Maximum friends limit reached.
    MaxFriends,
    /// Maximum group DM recipients limit reached.
    MaxGroupDmRecipients,
    /// Maximum group DMs limit reached.
    MaxGroupDms,
    /// Maximum guild channels limit reached.
    MaxGuildChannels,
    /// Maximum guild members limit reached.
    MaxGuildMembers,
    /// Maximum guild roles limit reached.
    MaxGuildRoles,
    /// Maximum guilds limit reached.
    MaxGuilds,
    /// Maximum invites limit reached.
    MaxInvites,
    /// Maximum pack expressions limit reached.
    MaxPackExpressions,
    /// Maximum packs limit reached.
    MaxPacks,
    /// Maximum pins per channel limit reached.
    MaxPinsPerChannel,
    /// Maximum reactions limit reached.
    MaxReactions,
    /// Maximum stickers limit reached.
    MaxStickers,
    /// Maximum webhooks per channel limit reached.
    MaxWebhooksPerChannel,
    /// Maximum webhooks per guild limit reached.
    MaxWebhooksPerGuild,
    /// Maximum webhooks limit reached.
    MaxWebhooks,
    /// NCMEC report has already been submitted.
    NcmecAlreadySubmitted,
    /// NCMEC report submission failed.
    NcmecSubmissionFailed,
    /// Error processing media metadata.
    MediaMetadataError,
    /// Missing access to this resource.
    MissingAccess,
    /// Missing access control list entry.
    MissingAcl,
    /// Missing authorization header.
    MissingAuthorization,
    /// Missing client secret.
    MissingClientSecret,
    /// Missing OAuth admin scope.
    MissingOauthAdminScope,
    /// Missing required OAuth fields.
    MissingOauthFields,
    /// Missing required OAuth scope.
    MissingOauthScope,
    /// Missing required permissions.
    MissingPermissions,
    /// Missing redirect URI.
    MissingRedirectUri,
    /// No active call in this channel.
    NoActiveCall,
    /// No active subscription.
    NoActiveSubscription,
    /// No passkeys registered for this account.
    NoPasskeysRegistered,
    /// No pending deletion for this account.
    NoPendingDeletion,
    /// No users with this Fluxertag exist.
    NoUsersWithFluxertagExist,
    /// No Visionary slots available.
    NoVisionarySlotsAvailable,
    /// You are not friends with this user.
    NotFriendsWithUser,
    /// You are not the owner of this admin API key.
    NotOwnerOfAdminApiKey,
    /// NSFW content is age restricted.
    NsfwContentAgeRestricted,
    /// Access to this pack is denied.
    PackAccessDenied,
    /// Passkey authentication failed.
    PasskeyAuthenticationFailed,
    /// Passkeys are disabled.
    PasskeysDisabled,
    /// This phone number is already in use.
    PhoneAlreadyUsed,
    /// Phone verification rate limit exceeded.
    PhoneRateLimitExceeded,
    /// Phone number required for SMS MFA.
    PhoneRequiredForSmsMfa,
    /// Phone verification is required.
    PhoneVerificationRequired,
    /// Premium purchase is blocked.
    PremiumPurchaseBlocked,
    /// Preview image must be JPEG format.
    PreviewMustBeJpeg,
    /// Processing failed.
    ProcessingFailed,
    /// You are being rate limited.
    RateLimited,
    /// Redirect URI required for non-bot application.
    RedirectUriRequiredForNonBot,
    /// This report has already been resolved.
    ReportAlreadyResolved,
    /// You are banned from submitting reports.
    ReportBanned,
    /// Session token mismatch.
    SessionTokenMismatch,
    /// You are being rate limited by slowmode.
    SlowmodeRateLimited,
    /// SMS MFA is not enabled.
    SmsMfaNotEnabled,
    /// SMS MFA requires TOTP to be enabled first.
    SmsMfaRequiresTotp,
    /// SMS verification is unavailable.
    SmsVerificationUnavailable,
    /// Single sign-on is required.
    SsoRequired,
    /// Stream key does not match channel.
    StreamKeyChannelMismatch,
    /// Stream key scope mismatch.
    StreamKeyScopeMismatch,
    /// Stream thumbnail payload is empty.
    StreamThumbnailPayloadEmpty,
    /// Stripe payment error.
    StripeError,
    /// Gift redemption already in progress.
    StripeGiftRedemptionInProgress,
    /// Invalid Stripe product.
    StripeInvalidProduct,
    /// Invalid Stripe product configuration.
    StripeInvalidProductConfiguration,
    /// No active Stripe subscription.
    StripeNoActiveSubscription,
    /// No Stripe purchase history.
    StripeNoPurchaseHistory,
    /// No Stripe subscription.
    StripeNoSubscription,
    /// Stripe payment not available.
    StripePaymentNotAvailable,
    /// Subscription is already being cancelled.
    StripeSubscriptionAlreadyCanceling,
    /// Subscription is not being cancelled.
    StripeSubscriptionNotCanceling,
    /// Subscription period end date is missing.
    StripeSubscriptionPeriodEndMissing,
    /// Stripe webhook not available.
    StripeWebhookNotAvailable,
    /// Invalid Stripe webhook signature.
    StripeWebhookSignatureInvalid,
    /// Missing Stripe webhook signature.
    StripeWebhookSignatureMissing,
    /// Sudo mode is required for this action.
    SudoModeRequired,
    /// This tag is already taken.
    TagAlreadyTaken,
    /// Temporary invite requires presence tracking.
    TemporaryInviteRequiresPresence,
    /// Test harness is disabled.
    TestHarnessDisabled,
    /// Test harness access is forbidden.
    TestHarnessForbidden,
    /// Two-factor authentication is not enabled.
    TwoFaNotEnabled,
    /// Two-factor authentication is required.
    TwoFactorRequired,
    /// Unauthorized.
    Unauthorized,
    /// Unclaimed accounts cannot accept friend requests.
    UnclaimedAccountCannotAcceptFriendRequests,
    /// Unclaimed accounts cannot add reactions.
    UnclaimedAccountCannotAddReactions,
    /// Unclaimed accounts cannot create applications.
    UnclaimedAccountCannotCreateApplications,
    /// Unclaimed accounts cannot create beta codes.
    UnclaimedAccountCannotCreateBetaCodes,
    /// Unclaimed accounts cannot join group DMs.
    UnclaimedAccountCannotJoinGroupDms,
    /// Unclaimed accounts cannot join one-on-one voice calls.
    UnclaimedAccountCannotJoinOneOnOneVoiceCalls,
    /// Unclaimed accounts cannot join voice channels.
    UnclaimedAccountCannotJoinVoiceChannels,
    /// Unclaimed accounts cannot make purchases.
    UnclaimedAccountCannotMakePurchases,
    /// Unclaimed accounts cannot send direct messages.
    UnclaimedAccountCannotSendDirectMessages,
    /// Unclaimed accounts cannot send friend requests.
    UnclaimedAccountCannotSendFriendRequests,
    /// Unclaimed accounts cannot send messages.
    UnclaimedAccountCannotSendMessages,
    /// Unknown beta code.
    UnknownBetaCode,
    /// Unknown channel.
    UnknownChannel,
    /// Unknown emoji.
    UnknownEmoji,
    /// Unknown saved media.
    #[serde(rename = "UNKNOWN_FAVORITE_MEME")]
    UnknownSavedMedia,
    /// Unknown gift code.
    UnknownGiftCode,
    /// Unknown guild.
    UnknownGuild,
    /// Unknown data harvest.
    UnknownHarvest,
    /// Unknown invite.
    UnknownInvite,
    /// Unknown member.
    UnknownMember,
    /// Unknown message.
    UnknownMessage,
    /// Unknown pack.
    UnknownPack,
    /// Unknown report.
    UnknownReport,
    /// Unknown role.
    UnknownRole,
    /// Unknown sticker.
    UnknownSticker,
    /// Unknown suspicious flag.
    UnknownSuspiciousFlag,
    /// Unknown user flag.
    UnknownUserFlag,
    /// Unknown user.
    UnknownUser,
    /// Unknown voice region.
    UnknownVoiceRegion,
    /// Unknown voice server.
    UnknownVoiceServer,
    /// Unknown WebAuthn credential.
    UnknownWebauthnCredential,
    /// Unknown webhook.
    UnknownWebhook,
    /// Unsupported response type.
    UnsupportedResponseType,
    /// Resource update failed.
    UpdateFailed,
    /// User is banned from this guild.
    UserBannedFromGuild,
    /// User IP is banned from this guild.
    UserIpBannedFromGuild,
    /// User is not in a voice channel.
    UserNotInVoice,
    /// User owns guilds and cannot perform this action.
    UserOwnsGuilds,
    /// Voice channel is full.
    VoiceChannelFull,
    /// WebAuthn credential limit reached.
    WebauthnCredentialLimitReached,
}
