use neptunium_model::id::Id;
use neptunium_model::id::marker::GuildMarker;
use neptunium_model::time::duration::{Duration, Millis};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

mod branding;
mod limits;

pub use branding::*;
pub use limits::*;

#[derive(Copy, Clone, Debug)]
pub struct InstanceDiscoveryDocument;

impl Endpoint for InstanceDiscoveryDocument {
    type Response = InstanceDiscoveryDocumentResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .use_authorization_token(false)
            .method(Method::GET)
            .path(String::from("/.well-known/fluxer"))
            .build()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentEndpoints {
    /// Base URL for authenticated API requests.
    pub api: String,
    /// Base URL for client API requests.
    pub api_client: String,
    /// Base URL for public API requests.
    pub api_public: String,
    /// WebSocket URL for the gateway.
    pub gateway: String,
    /// Base URL for the media proxy.
    pub media: String,
    /// Attachment upload relay base URL
    pub upload_relay: Option<String>,
    /// Base URL for static assets (avatars, emojis, etc.).
    pub static_cdn: String,
    /// Base URL for the marketing website.
    pub marketing: String,
    /// Base URL for the admin panel.
    pub admin: String,
    /// Base URL for invite links.
    pub invite: String,
    /// Base URL for gift links.
    pub gift: String,
    /// Base URL for the web application.
    pub webapp: String,
    /// Documentation site base URL
    pub docs: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase", tag = "provider")]
pub enum InstanceDiscoveryDocumentCaptchaConfig {
    Altcha,
    None,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct InstanceDiscoveryDocumentFeatures {
    pub sms_mfa_enabled: bool,
    pub voice_enabled: bool,
    pub stripe_enabled: bool,
    pub self_hosted: bool,
    // TODO: Check what's up with https://github.com/fluxerapp/fluxer/blob/03813bbe17db008452f0f1be3090a7d2970a5447/packages/api/src/instance/InstanceController.tsx#L86
    /// This is documented as "required" in the documentation, but appears to be missing.
    pub manual_review_enabled: Option<bool>,
    /// Undocumented so far.
    pub presigned_attachment_uploads: bool,
}

/// Single sign-on configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentSSOConfiguration {
    pub enabled: bool,
    pub enforced: bool,
    pub display_name: Option<String>,
    pub redirect_uri: String,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InstanceDiscoveryDocumentGifProviders {
    Klipy,
    Tenor,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentGifProvider {
    pub provider: InstanceDiscoveryDocumentGifProviders,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentPushNotificationDeliveryMode {
    Direct,
    ExternalRelay,
    Hybrid,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentPushNotificationConfig {
    /// The Web Push VAPID public key, or null when Web Push isn't set up
    pub public_vapid_key: Option<String>,
    /// How push notifications get to devices, direct, external_relay or hybrid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<InstanceDiscoveryDocumentPushNotificationDeliveryMode>,
    /// The API path to call to get the payload of a push event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_resolution_path: Option<String>,
    /// The URL of the external push relay, or null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_relay_url: Option<String>,
    /// Hashes of the keys the external push relay signs with, or null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_signing_key_hashes: Option<Vec<String>>,
}

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct InstanceDiscoveryDocumentGateway {
//     #[serde(rename = "session_retry_min_ms")]
//     pub session_retry_minimum: Duration<Millis>,
//     #[serde(rename = "session_retry_max_ms")]
//     pub session_retry_maximum: Duration<Millis>,
//     #[serde(rename = "session_retry_jitter_ms")]
//     pub session_retry_jitter: Duration<Millis>,
// }
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct InstanceDiscoveryDocumentFederationConfig {
//     pub enabled: bool,
//     /// Federation protocol version.
//     pub version: u64,
// }

// #[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
// pub enum InstanceDiscoveryDocumentPublicKeyAlgorithm {
//     #[serde(rename = "x25519")]
//     X25519,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct InstanceDiscoveryDocumentPublicKey {
//     pub id: String,
//     pub algorithm: InstanceDiscoveryDocumentPublicKeyAlgorithm,
//     pub public_key_base64: String,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct InstanceDiscoveryDocumentOauth2 {
//     pub authorization_endpoint: String,
//     pub token_endpoint: String,
//     pub userinfo_endpoint: String,
//     pub scopes_supported: Vec<String>,
// }

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InstanceDiscoveryDocumentRegistrationMode {
    /// Anyone can register and gets a session at once
    Open,
    /// New accounts wait for admin approval before they can sign in
    Approval,
    /// Public registration is refused
    Closed,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentRegistrationPolicy {
    pub mode: InstanceDiscoveryDocumentRegistrationMode,
    /// Whether register an account accepts an admin-issued registration_url_code,
    /// which works even while mode is closed
    pub admin_registration_urls_enabled: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentCommunityPolicy {
    /// Whether the instance runs as one community that every account joins
    single_community: bool,
    /// The ID of the single community's guild, or null when single_community is false
    single_community_guild_id: Option<Id<GuildMarker>>,
    /// Whether direct messages are off for the whole instance
    direct_messages_disabled: bool,
    /// Whether friend requests are off for the whole instance
    #[serde(skip_serializing_if = "Option::is_none")]
    friends_disabled: Option<bool>,
    /// Whether people can use personal notes. Missing means they can.
    #[serde(skip_serializing_if = "Option::is_none")]
    personal_notes_enabled: Option<bool>,
    /// Whether every account can create communities.
    /// When false, only admins and accounts with feature_guild_create can.
    guild_create_access: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentServiceAvailability {
    /// Whether the GIF picker is available
    gif_enabled: bool,
    /// Whether YouTube link previews are available
    youtube_enabled: bool,
    /// Whether Bluesky connections are available
    bluesky_enabled: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentAgePolicyAction {
    /// Needs an age check
    Restrict,
    /// Refuse access
    Block,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentAgePolicyRegion {
    /// The ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// The ISO 3166-2 subdivision code, or null for the whole country
    pub region_code: Option<String>,
    pub action: InstanceDiscoveryDocumentAgePolicyAction,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentAgePolicy {
    /// Regions with an age policy
    pub geos: Vec<InstanceDiscoveryDocumentAgePolicyRegion>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentNotice {
    pub id: String,
    pub message: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentResponse {
    /// The protocol generation the instance speaks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codename: Option<String>,
    /// Version of the API server code.
    pub api_code_version: u64,
    pub endpoints: InstanceDiscoveryDocumentEndpoints,
    pub captcha: InstanceDiscoveryDocumentCaptchaConfig,
    pub features: InstanceDiscoveryDocumentFeatures,
    pub gif: InstanceDiscoveryDocumentGifProvider,
    pub sso: InstanceDiscoveryDocumentSSOConfiguration,
    /// Who can register
    pub registration: InstanceDiscoveryDocumentRegistrationPolicy,
    /// Community and direct message policy
    pub community: InstanceDiscoveryDocumentCommunityPolicy,
    /// Optional integrations that are available
    pub services: InstanceDiscoveryDocumentServiceAvailability,
    /// Limit rules and trait names
    pub limits: InstanceDiscoveryDocumentLimits,
    /// Web Push identity
    pub push: InstanceDiscoveryDocumentPushNotificationConfig,
    /// Branding, setup state and legal links
    pub app_public: InstanceDiscoveryDocumentAppPublic,
    /// Regional age policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_policy: Option<InstanceDiscoveryDocumentAgePolicy>,
    /// The current notice from the instance admins, or null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice: Option<InstanceDiscoveryDocumentNotice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_badges: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_badges_partial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_badges_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_in_guild_badge_images: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_built_in_guild_badges: Option<Vec<()>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_origins: Option<Vec<()>>,
}
