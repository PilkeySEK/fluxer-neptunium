use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentAccountIdentityMode {
    /// People register and sign in with an email address
    Email,
    /// People register and sign in with a username,
    /// and the instance collects no email address
    Username,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentAccountTagStyle {
    /// Usernames are unique and have no tag, and every account that
    /// isn't a bot has the discriminator 0000
    None,
    /// Every account gets a random tag that can be changed
    Random,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentInstantAccountAvailability {
    /// Sign-up can create an instant account
    Available,
    /// An admin turned instant accounts off
    Disabled,
    /// Instant accounts are on, but the CAPTCHA they need is off
    CaptchaOff,
    /// Public registration is closed
    RegistrationClosed,
    /// New accounts wait for admin approval
    ApprovalRequired,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentInteractionTier {
    Legacy,
    Enforced,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentGuildJoinGate {
    Off,
    VisitsCapable,
    Always,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentEmailDeliveryState {
    /// The instance sends email
    On,
    /// The instance uses usernames and never sends email
    OffUsernameMode,
    /// An admin or the configuration turned email off
    OffDisabled,
    /// Email is on, but the SMTP settings or sender address are incomplete
    OffIncomplete,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentAccountVerificationBasis {
    ConfirmedEmail,
    LocalCredential,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct InstanceDiscoveryDocumentFeatures {
    /// Whether voice and video calls are enabled
    pub voice_enabled: bool,
    /// Whether premium purchases through Stripe billing are available
    pub stripe_enabled: bool,
    /// Whether the instance has a premium tier
    pub premium_enabled: bool,
    /// Whether existing Stripe subscriptions can be managed and billed
    pub stripe_serviceable: bool,
    /// Whether the instance is self-hosted
    pub self_hosted: bool,
    /// Whether you can request presigned attachment upload URLs
    pub presigned_attachment_uploads: bool,
    /// Whether the instance sends email
    pub emails_enabled: bool,
    /// Deprecated. Always false
    pub phone_verification_enabled: bool,
    /// Whether the Explore community directory is available, true when missing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_enabled: Option<bool>,
    /// How people sign in
    pub account_identity: InstanceDiscoveryDocumentAccountIdentityMode,
    /// How usernames are tagged
    pub tag_style: InstanceDiscoveryDocumentAccountTagStyle,
    /// Whether the desktop app can load downloadable modules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_modules_enabled: Option<bool>,
    /// Hours between a deletion request and permanent deletion (1-8,760)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_deletion_grace_period_hours: Option<u16>,
    /// Whether sign-in starts with identify an account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_first: Option<bool>,
    /// Whether register an account accepts an account with only a display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_accounts: Option<bool>,
    /// Whether send an email code works
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_codes: Option<bool>,
    /// Whether accounts set their date of birth after sign-up
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth_step: Option<bool>,
    /// Whether sign-up can create an instant account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_account_availability: Option<InstanceDiscoveryDocumentInstantAccountAvailability>,
    /// How the account tiers apply, legacy or enforced
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_tier: Option<InstanceDiscoveryDocumentInteractionTier>,
    /// When joining a guild needs a verified account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_join_gate: Option<InstanceDiscoveryDocumentGuildJoinGate>,
    /// Whether the instance sends email and why not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_delivery_state: Option<InstanceDiscoveryDocumentEmailDeliveryState>,
    /// What an account needs to be verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_basis: Option<InstanceDiscoveryDocumentAccountVerificationBasis>,
}
