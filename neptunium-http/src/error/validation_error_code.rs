use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiValidationErrorCode {
    /// Accent colour has been changed too many times recently.
    AccentColorChangedTooManyTimes,
    /// Account is already verified.
    AccountAlreadyVerified,
    /// AFK channel must be in the same guild.
    AfkChannelMustBeInGuild,
    /// AFK channel must be a voice channel.
    AfkChannelMustBeVoice,
    /// All channels must belong to the same guild.
    AllChannelsMustBelongToGuild,
    /// Animated avatars require premium.
    AnimatedAvatarsRequirePremium,
    /// Animated guild banners require the feature to be enabled.
    AnimatedGuildBannerRequiresFeature,
    /// At least one entry is required.
    AtLeastOneEntryIsRequired,
    /// At least one recipient is required.
    AtLeastOneRecipientRequired,
    /// Attachment fields are required.
    AttachmentFieldsRequired,
    /// Attachment ID was not found in the message.
    AttachmentIdNotFoundInMessage,
    /// Attachment IDs must be valid integers.
    AttachmentIdsMustBeValidIntegers,
    /// Attachment metadata provided without files.
    AttachmentMetadataWithoutFiles,
    /// Attachment must be an image.
    AttachmentMustBeImage,
    /// Attachments metadata is required when uploading files.
    AttachmentsMetadataRequiredWhenUploading,
    /// Attachments are not allowed for this message type.
    AttachmentsNotAllowedForMessage,
    /// Avatar has been changed too many times recently.
    AvatarChangedTooManyTimes,
    /// Banner has been changed too many times recently.
    BannerChangedTooManyTimes,
    /// Banners require premium.
    BannersRequirePremium,
    /// Invalid base64 length.
    Base64LengthInvalid,
    /// Bio has been changed too many times recently.
    BioChangedTooManyTimes,
    /// Bucket is required.
    BucketIsRequired,
    /// Cannot add yourself to a group DM.
    CannotAddYourselfToGroupDm,
    /// Cannot delete more than 100 messages at once.
    #[serde(rename = "CANNOT_DELETE_MORE_THAN_100_MESSAGES")]
    CannotDeleteMoreThan100Messages,
    /// Cannot send a direct message to yourself.
    CannotDmYourself,
    /// Cannot leave guild as the owner.
    CannotLeaveGuildAsOwner,
    /// Cannot position channel relative to itself.
    CannotPositionChannelRelativeToItself,
    /// Cannot preload more than 100 channels.
    #[serde(rename = "CANNOT_PRELOAD_MORE_THAN_100_CHANNELS")]
    CannotPreloadMoreThan100Channels,
    /// Cannot reference attachments without providing attachments.
    CannotReferenceAttachmentsWithoutAttachments,
    /// Cannot reorder the everyone role.
    CannotReorderEveryoneRole,
    /// Cannot reply to a system message.
    CannotReplyToSystemMessage,
    /// Cannot set hoist for the everyone role.
    CannotSetHoistForEveryoneRole,
    /// Cannot specify both before and after parameters.
    CannotSpecifyBothBeforeAndAfter,
    /// Cannot use the same role as preceding.
    CannotUseSameRoleAsPreceding,
    /// Categories cannot have a parent channel.
    CategoriesCannotHaveParentChannel,
    /// Categories cannot have parents.
    CategoriesCannotHaveParents,
    /// Changing discriminator requires premium.
    ChangingDiscriminatorRequiresPremium,
    /// Channel does not exist.
    ChannelDoesNotExist,
    /// Channel ID is required.
    ChannelIdIsRequired,
    /// Channel must be a DM or group DM.
    ChannelMustBeDmOrGroupDm,
    /// Channel must be a voice channel.
    ChannelMustBeVoice,
    /// Channel name is empty after normalisation.
    ChannelNameEmptyAfterNormalization,
    /// Channel was not found.
    ChannelNotFound,
    /// Colour value is too high.
    ColorValueTooHigh,
    /// Colour value is too low.
    ColorValueTooLow,
    /// Content exceeds maximum length.
    ContentExceedsMaxLength,
    /// Context channel or guild ID is required.
    ContextChannelOrGuildIdRequired,
    /// Custom emoji was not found.
    CustomEmojiNotFound,
    /// Custom emojis require premium when used outside their source.
    CustomEmojisRequirePremiumOutsideSource,
    /// Custom sticker was not found.
    CustomStickerNotFound,
    /// Custom stickers in DMs require premium.
    CustomStickersInDmsRequirePremium,
    /// Custom stickers require premium when used outside their source.
    CustomStickersRequirePremiumOutsideSource,
    /// Discriminator has an invalid format.
    DiscriminatorInvalidFormat,
    /// Discriminator is out of valid range.
    DiscriminatorOutOfRange,
    /// Duplicate attachment IDs are not allowed.
    DuplicateAttachmentIdsNotAllowed,
    /// Duplicate file index.
    DuplicateFileIndex,
    /// Duplicate recipients are not allowed.
    DuplicateRecipientsNotAllowed,
    /// Voice message attachment must be audio.
    VoiceMessagesAttachmentMustBeAudio,
    /// Voice message attachment waveform is required.
    VoiceMessagesAttachmentWaveformRequired,
    /// Voice message attachment duration is required.
    VoiceMessagesAttachmentDurationRequired,
    /// Voice messages cannot have content.
    VoiceMessagesCannotHaveContent,
    /// Voice messages cannot have embeds.
    VoiceMessagesCannotHaveEmbeds,
    /// Voice messages cannot have saved media.
    #[serde(rename = "VOICE_MESSAGES_CANNOT_HAVE_FAVORITE_MEMES")]
    VoiceMessagesCannotHaveSavedMedia,
    /// Voice messages cannot have stickers.
    VoiceMessagesCannotHaveStickers,
    /// Voice message duration exceeds limit.
    VoiceMessagesDurationExceedsLimit,
    /// Voice messages require a single attachment.
    VoiceMessagesRequireSingleAttachment,
    /// Email address is already in use.
    EmailAlreadyInUse,
    /// Email address is required.
    EmailIsRequired,
    /// Email address length is invalid.
    EmailLengthInvalid,
    /// Email must be changed via verification token.
    EmailMustBeChangedViaToken,
    /// Email verification token has expired.
    EmailTokenExpired,
    /// Embed index is out of bounds.
    EmbedIndexOutOfBounds,
    /// Embed splash requires the feature to be enabled.
    EmbedSplashRequiresFeature,
    /// Embeds exceed maximum character count.
    EmbedsExceedMaxCharacters,
    /// Emoji requires guild or pack access.
    EmojiRequiresGuildOrPackAccess,
    /// Failed to parse multipart form data.
    FailedToParseMultipartFormData,
    /// Failed to parse multipart payload.
    FailedToParseMultipartPayload,
    /// Failed to upload image.
    FailedToUploadImage,
    /// Saved media name is required.
    #[serde(rename = "FAVORITE_MEME_NAME_REQUIRED")]
    SavedMediaNameRequired,
    /// Saved media was not found.
    #[serde(rename = "FAVORITE_MEME_NOT_FOUND")]
    SavedMediaNotFound,
    /// File index exceeds maximum.
    FileIndexExceedsMaximum,
    /// File not found for scanning.
    FileNotFoundForScanning,
    /// File was not found.
    FileNotFound,
    /// Filename is empty after normalisation.
    FilenameEmptyAfterNormalization,
    /// Filename contains invalid characters.
    FilenameInvalidCharacters,
    /// Filename length is invalid.
    FilenameLengthInvalid,
    /// Filename mismatch for attachment.
    FilenameMismatchForAttachment,
    /// Forward messages cannot contain content.
    ForwardMessagesCannotContainContent,
    /// Forward reference requires channel and message.
    ForwardReferenceRequiresChannelAndMessage,
    /// Display name cannot contain reserved terms.
    GlobalNameCannotContainReservedTerms,
    /// Display name length is invalid.
    GlobalNameLengthInvalid,
    /// Display name is a reserved value.
    GlobalNameReservedValue,
    /// Guild banner requires the feature to be enabled.
    GuildBannerRequiresFeature,
    /// Guild ID must match referenced message.
    GuildIdMustMatchReferencedMessage,
    /// Image size exceeds limit.
    ImageSizeExceedsLimit,
    /// Integer is out of 64-bit range.
    IntegerOutOfInt64Range,
    /// Snowflake is out of valid range.
    SnowflakeOutOfRange,
    /// Invalid audit log reason.
    InvalidAuditLogReason,
    /// Invalid base64 format.
    InvalidBase64Format,
    /// Invalid channel ID.
    InvalidChannelId,
    /// Invalid channel.
    InvalidChannel,
    /// Invalid code.
    InvalidCode,
    /// Invalid date of birth format.
    InvalidDateOfBirthFormat,
    /// Invalid datetime for scheduled send.
    InvalidDatetimeForScheduledSend,
    /// Invalid email address.
    InvalidEmailAddress,
    /// Invalid email format.
    InvalidEmailFormat,
    /// Invalid email local part.
    InvalidEmailLocalPart,
    /// Invalid email or password.
    InvalidEmailOrPassword,
    /// Invalid email verification token.
    InvalidEmailToken,
    /// Invalid file field name.
    InvalidFileFieldName,
    /// Invalid format.
    InvalidFormat,
    /// Invalid image data.
    InvalidImageData,
    /// Invalid image format.
    InvalidImageFormat,
    /// Invalid integer format.
    InvalidIntegerFormat,
    /// Invalid snowflake format.
    InvalidSnowflakeFormat,
    /// Invalid ISO timestamp.
    InvalidIsoTimestamp,
    /// Invalid job ID.
    InvalidJobId,
    /// Invalid JSON in `payload_json` field.
    InvalidJsonInPayloadJson,
    /// Invalid message data.
    InvalidMessageData,
    /// Invalid MFA code.
    InvalidMfaCode,
    /// Invalid or already used beta code.
    InvalidOrAlreadyUsedBetaCode,
    /// Invalid or expired authorisation ticket.
    InvalidOrExpiredAuthorizationTicket,
    /// Invalid or expired authorisation token.
    InvalidOrExpiredAuthorizationToken,
    /// Invalid or expired password reset token.
    InvalidOrExpiredResetToken,
    /// Invalid or expired revert token.
    InvalidOrExpiredRevertToken,
    /// Invalid or expired ticket.
    InvalidOrExpiredTicket,
    /// Invalid or expired verification token.
    InvalidOrExpiredVerificationToken,
    /// Invalid or restricted RTC region.
    InvalidOrRestrictedRtcRegion,
    /// Invalid parent channel.
    InvalidParentChannel,
    /// Invalid password.
    InvalidPassword,
    /// Invalid proof token.
    InvalidProofToken,
    /// Invalid role ID.
    InvalidRoleId,
    /// Invalid RTC region.
    InvalidRtcRegion,
    /// Invalid scheduled message payload.
    InvalidScheduledMessagePayload,
    /// Invalid snowflake.
    InvalidSnowflake,
    /// Invalid timeout value.
    InvalidTimeoutValue,
    /// Invalid timezone identifier.
    InvalidTimezoneIdentifier,
    /// Invalid URL format.
    InvalidUrlFormat,
    /// Invalid URL or attachment format.
    InvalidUrlOrAttachmentFormat,
    /// Invalid verification code.
    InvalidVerificationCode,
    /// Invite splash requires the feature to be enabled.
    InviteSplashRequiresFeature,
    /// Job ID is required.
    JobIdIsRequired,
    /// Job has already been processed.
    JobIsAlreadyProcessed,
    /// Job was not found.
    JobNotFound,
    /// Media is already in saved media.
    #[serde(rename = "MEDIA_ALREADY_IN_FAVORITE_MEMES")]
    MediaAlreadyInSavedMedia,
    /// Message IDs cannot be empty.
    MessageIdsCannotBeEmpty,
    /// Messages array is required and cannot be empty.
    MessagesArrayRequiredAndNotEmpty,
    /// Messages with snapshots cannot be edited.
    MessagesWithSnapshotsCannotBeEdited,
    /// Multiple files for the same index are not allowed.
    MultipleFilesForIndexNotAllowed,
    /// Must agree to terms of service and privacy policy.
    MustAgreeToTosAndPrivacyPolicy,
    /// Must be minimum age to use this service.
    MustBeMinimumAge,
    /// Must enable 2FA before requiring it for moderators.
    #[serde(rename = "MUST_ENABLE_2FA_BEFORE_REQUIRING_FOR_MODS")]
    MustEnable2faBeforeRequiringForMods,
    /// Must have an email to change it.
    MustHaveEmailToChangeIt,
    /// Must start session before sending.
    MustStartSessionBeforeSending,
    /// Name is empty after normalisation.
    NameEmptyAfterNormalization,
    /// New email must be different from current email.
    NewEmailMustBeDifferent,
    /// No file provided for attachment metadata.
    NoFileForAttachmentMetadata,
    /// No file provided for attachment.
    NoFileForAttachment,
    /// No metadata provided for file.
    NoMetadataForFile,
    /// No new email has been requested.
    NoNewEmailRequested,
    /// No original email on record.
    NoOriginalEmailOnRecord,
    /// No valid media in message.
    NoValidMediaInMessage,
    /// Not a valid Unicode emoji.
    NotAValidUnicodeEmoji,
    /// Original email is already verified.
    OriginalEmailAlreadyVerified,
    /// Original email must be verified first.
    OriginalEmailMustBeVerifiedFirst,
    /// Original verification is not required.
    OriginalVerificationNotRequired,
    /// Parent channel is not in the guild.
    ParentChannelNotInGuild,
    /// Parent channel must be a category.
    ParentMustBeCategory,
    /// Parse and users/roles cannot be used together.
    ParseAndUsersOrRolesCannotBeUsedTogether,
    /// Password is too common.
    PasswordIsTooCommon,
    /// Password length is invalid.
    PasswordLengthInvalid,
    /// Password is not set.
    PasswordNotSet,
    /// `payload_json` is required for multipart requests.
    PayloadJsonRequiredForMultipart,
    /// Phone number has an invalid format.
    PhoneNumberInvalidFormat,
    /// Preceding channel must share the same parent.
    PrecedingChannelMustShareParent,
    /// Preceding channel is not in the guild.
    PrecedingChannelNotInGuild,
    /// Preceding role is not in the guild.
    PrecedingRoleNotInGuild,
    /// Premium is required for custom emoji.
    PremiumRequiredForCustomEmoji,
    /// Pronouns have been changed too many times recently.
    PronounsChangedTooManyTimes,
    /// Recipient IDs cannot be empty.
    RecipientIdsCannotBeEmpty,
    /// Recipient IDs must be strings.
    RecipientIdsMustBeStrings,
    /// Recipient IDs must be valid snowflakes.
    RecipientIdsMustBeValidSnowflakes,
    /// Referenced attachment was not found.
    ReferencedAttachmentNotFound,
    /// Rows field is required.
    RowsIsRequired,
    /// Scheduled messages must be within 30 days.
    #[serde(rename = "SCHEDULED_MESSAGES_MAX_30_DAYS")]
    ScheduledMessagesMax30Days,
    /// Scheduled time must be in the future.
    ScheduledTimeMustBeFuture,
    /// Session has timed out.
    SessionTimeout,
    /// Size in bytes must be a valid integer.
    SizeBytesMustBeValidInteger,
    /// String must be exactly the required length.
    StringLengthExact,
    /// String length is invalid.
    StringLengthInvalid,
    /// System channel must be in the guild.
    SystemChannelMustBeInGuild,
    /// System channel must be a text channel.
    SystemChannelMustBeText,
    /// Tag is already taken.
    TagAlreadyTaken,
    /// This vanity URL is already taken.
    ThisVanityUrlIsAlreadyTaken,
    /// Ticket has already been completed.
    TicketAlreadyCompleted,
    /// Timeout cannot exceed 365 days.
    #[serde(rename = "TIMEOUT_CANNOT_EXCEED_365_DAYS")]
    TimeoutCannotExceed365Days,
    /// Too many embeds.
    TooManyEmbeds,
    /// Too many files.
    TooManyFiles,
    /// Too many users with this username.
    TooManyUsersWithThisUsername,
    /// Too many users with this username, try a different one.
    TooManyUsersWithUsernameTryDifferent,
    /// Unclaimed accounts can only set email via verification token.
    UnclaimedAccountsCanOnlySetEmailViaToken,
    /// Unknown image format.
    UnknownImageFormat,
    /// Unresolved attachment URL.
    UnresolvedAttachmentUrl,
    /// Uploaded attachment was not found.
    UploadedAttachmentNotFound,
    /// URL length is invalid.
    UrlLengthInvalid,
    /// User does not have an email address.
    UserDoesNotHaveAnEmailAddress,
    /// User is not banned.
    UserIsNotBanned,
    /// User is not pending verification.
    UserIsNotPendingVerification,
    /// User must be a bot to be marked as a system user.
    UserMustBeABotToBeMarkedAsASystemUser,
    /// User is not in the channel.
    UserNotInChannel,
    /// Username cannot contain reserved terms.
    UsernameCannotContainReservedTerms,
    /// Username has been changed too many times recently.
    UsernameChangedTooManyTimes,
    /// Username contains invalid characters.
    UsernameInvalidCharacters,
    /// Username length is invalid.
    UsernameLengthInvalid,
    /// Username is a reserved value.
    UsernameReservedValue,
    /// Value must be an integer in the valid range.
    ValueMustBeIntegerInRange,
    /// Value is too small.
    ValueTooSmall,
    /// Vanity URL code is already taken.
    VanityUrlCodeAlreadyTaken,
    /// Vanity URL code cannot contain fluxer.
    VanityUrlCodeCannotContainFluxer,
    /// Vanity URL code length is invalid.
    VanityUrlCodeLengthInvalid,
    /// Vanity URL contains invalid characters.
    VanityUrlInvalidCharacters,
    /// Vanity URL requires the feature to be enabled.
    VanityUrlRequiresFeature,
    /// Verification code has expired.
    VerificationCodeExpired,
    /// Verification code was not issued.
    VerificationCodeNotIssued,
    /// Visionary subscription required for bot discriminator.
    VisionaryRequiredForBotDiscriminator,
    /// Visionary subscription required for discriminator.
    VisionaryRequiredForDiscriminator,
    /// Voice channels cannot be positioned above text channels.
    VoiceChannelsCannotBeAboveTextChannels,
    /// Webhook name length is invalid.
    WebhookNameLengthInvalid,
}
