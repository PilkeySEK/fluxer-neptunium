use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentAppPublicBranding {
    /// The instance name.
    pub product_name: String,
    /// App icon URL, or null.
    pub icon_url: Option<String>,
    /// Compact app symbol URL, or null.
    pub symbol_url: Option<String>,
    /// App logo URL, or null.
    pub logo_url: Option<String>,
    /// App wordmark URL, or null.
    pub wordmark_url: Option<String>,
    /// Favicon URL, or null.
    pub favicon_url: Option<String>,
    /// Favicon URL for a dark color scheme, or null to use `favicon_url`.
    pub favicon_dark_url: Option<String>,
    /// Brand color as lowercase #rrggbb, or null.
    pub theme_color: Option<String>,
    /// Status page URL, or null.
    pub status_page_url: Option<String>,
    /// Whether the status page is an Instatus page browsers can read.
    /// Clients show its incidents only when this is true.
    pub status_page_instatus: bool,
    /// Status page incident history URL, or null.
    pub status_page_incident_history_url: Option<String>,
    /// The name of the premium tier, Premium unless an admin sets one.
    pub premium_product_name: String,
    /// URL of a page about the premium tier, or null.
    pub premium_info_url: Option<String>,
    /// Image shown behind the sign-in screens, or null for the default.
    pub sign_in_backdrop_url: Option<String>,
    /// CSS the web and desktop app apply before the person's own theme,
    /// except on sign-in pages, or null.
    pub custom_css: Option<String>,
    /// Markdown shown when a file is over the upload limit, or null for the default text.
    pub upload_limit_message: Option<String>,
    /// Extra domains whose links open without a warning.
    /// Entries are hostnames, and a *. prefix also covers subdomains.
    pub trusted_domains: Vec<String>,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentAccountIdentityLockReason {
    /// Setup is finished.
    SetupConfigured,
    /// The instance had an admin, accounts or finished setup when
    /// its sign-in method was first set, so it stays on email.
    ExistingInstance,
    /// The instance has an admin account.
    AdminExists,
    /// The instance has accounts, none of them an admin.
    AccountsExist,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentAppPublicSetupState {
    /// Whether setup is finished.
    pub configured: bool,
    /// The admin app URL to finish setup in, or null.
    pub admin_url: Option<String>,
    /// Whether the sign-in method can no longer change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identity_locked: Option<bool>,
    /// One of the account identity lock reasons, or null while the method can change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identity_lock_reason: Option<InstanceDiscoveryDocumentAccountIdentityLockReason>,
    /// Whether register an account needs `setup_token`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_token_required: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentAppPublicLegal {
    /// Terms of service URL, or null.
    pub terms_url: Option<String>,
    /// Privacy policy URL, or null.
    pub privacy_url: Option<String>,
    /// Community guidelines URL, used by report flows, or null to hide guidelines links.
    pub guidelines_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentAppPublicRegistration {
    /// Whether sign-up asks for a date of birth, set in the admin accounts configuration.
    pub collect_date_of_birth: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentAppPublic {
    /// Instance name and images.
    pub branding: InstanceDiscoveryDocumentAppPublicBranding,
    /// Setup state.
    pub setup: InstanceDiscoveryDocumentAppPublicSetupState,
    /// Legal document URLs.
    pub legal: InstanceDiscoveryDocumentAppPublicLegal,
    /// Extra sign-up fields.
    pub registration: InstanceDiscoveryDocumentAppPublicRegistration,
}
