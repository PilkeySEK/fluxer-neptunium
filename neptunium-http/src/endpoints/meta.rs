use neptunium_model::id::Id;
use neptunium_model::id::marker::{GuildBadgeMarker, GuildMarker};
use neptunium_model::time::timestamp::Timestamp;
use neptunium_model::time::timestamp::representations::Iso8601;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

mod branding;
mod features;
mod limits;

pub use branding::*;
pub use features::*;
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
    /// Attachment upload relay base URL.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Documentation site base URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase", tag = "provider")]
pub enum InstanceDiscoveryDocumentCaptchaConfig {
    Altcha,
    None,
}

/// Single sign-on configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentSSOStatus {
    /// Whether SSO is turned on and its provider is fully configured.
    pub enabled: bool,
    /// Whether every user has to sign in with SSO.
    pub enforced: bool,
    /// Whether each SSO sign-in sets the username from the provider,
    /// so SSO accounts can't change it.
    pub username_from_provider: bool,
    /// The provider display name.
    pub display_name: Option<String>,
    /// The text for the SSO sign-in button, or null to use the app's default.
    pub button_label: Option<String>,
    /// The default redirect URI for the provider callback.
    pub redirect_uri: String,
    /// Not sent by the API, so check enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentGifProviders {
    Klipy,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentGifProvider {
    /// The machine name of the GIF provider.
    pub provider: InstanceDiscoveryDocumentGifProviders,
    /// The provider name to show.
    pub display_name: String,
    /// Whether you must show the provider's attribution next to GIF results.
    pub attribution_required: bool,
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
    /// The Web Push VAPID public key, or null when Web Push isn't set up.
    pub public_vapid_key: Option<String>,
    /// How push notifications get to devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<InstanceDiscoveryDocumentPushNotificationDeliveryMode>,
    /// The API path to call to get the payload of a push event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_resolution_path: Option<String>,
    /// The URL of the external push relay, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_relay_url: Option<String>,
    /// Hashes of the keys the external push relay signs with, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_signing_key_hashes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InstanceDiscoveryDocumentRegistrationMode {
    /// Anyone can register and gets a session at once.
    Open,
    /// New accounts wait for admin approval before they can sign in.
    Approval,
    /// Public registration is refused.
    Closed,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentRegistrationPolicy {
    pub mode: InstanceDiscoveryDocumentRegistrationMode,
    /// Whether register an account accepts an admin-issued registration url code,
    /// which works even while mode is closed.
    pub admin_registration_urls_enabled: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentCommunityPolicy {
    /// Whether the instance runs as one community that every account joins.
    single_community: bool,
    /// The ID of the single community's guild, or null when `single_community` is false.
    single_community_guild_id: Option<Id<GuildMarker>>,
    /// Whether direct messages are off for the whole instance.
    direct_messages_disabled: bool,
    /// Whether friend requests are off for the whole instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    friends_disabled: Option<bool>,
    /// Whether people can use personal notes. Missing means they can.
    #[serde(skip_serializing_if = "Option::is_none")]
    personal_notes_enabled: Option<bool>,
    /// Whether every account can create communities.
    /// When false, only admins and accounts with `feature_guild_create` can.
    guild_create_access: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentServiceAvailability {
    /// Whether the GIF picker is available.
    gif_enabled: bool,
    /// Whether Youtube link previews are available.
    youtube_enabled: bool,
    /// Whether Bluesky connections are available.
    bluesky_enabled: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentAgePolicyAction {
    /// Needs an age check.
    Restrict,
    /// Refuse access.
    Block,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentAgePolicyRegion {
    /// The ISO 3166-1 alpha-2 country code.
    pub country_code: String,
    /// The ISO 3166-2 subdivision code, or null for the whole country.
    pub region_code: Option<String>,
    pub action: InstanceDiscoveryDocumentAgePolicyAction,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentAgePolicy {
    /// Regions with an age policy.
    pub geos: Vec<InstanceDiscoveryDocumentAgePolicyRegion>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentNotice {
    /// The ID of this version of the notice, which changes when any other field changes.
    pub id: String,
    /// The notice text (1-1000 characters).
    pub message: String,
    /// An http or https link shown with the notice, or null.
    pub url: Option<String>,
    /// When to start showing the notice, or null to show it at once.
    pub starts_at: Option<Timestamp<Iso8601>>,
    /// When to stop showing the notice, or null for no end.
    pub ends_at: Option<Timestamp<Iso8601>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentGuildBadge {
    /// The ID of the badge.
    pub id: Id<GuildBadgeMarker>,
    /// The name of the badge.
    pub name: String,
    /// The tooltip text under the name, or null.
    pub description: Option<String>,
    /// The badge image as WebP from the media proxy.
    /// Add a size query parameter to get it at the size you draw it.
    pub asset_url: String,
    /// The https URL a click opens, or null.
    pub href: Option<String>,
    /// The display position as a 64-bit integer string.
    /// Older instances leave it out, and clients keep the `guild_badges` order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentBuiltinGuildBadgeImages {
    /// The image for the verified badge, as WebP from the media proxy, or null.
    pub verified: Option<String>,
    /// The image for the partnered badge, or null.
    pub partnered: Option<String>,
    /// The image for the discoverable badge, or null.
    pub discoverable: Option<String>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDiscoveryDocumentBuiltinGuildBadge {
    Verified,
    Partnered,
    Discoverable,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstanceDiscoveryDocumentResponse {
    /// The protocol generation the instance speaks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codename: Option<String>,
    /// Version of the API server code.
    pub api_code_version: u64,
    /// Public service URLs.
    pub endpoints: InstanceDiscoveryDocumentEndpoints,
    /// CAPTCHA configuration.
    pub captcha: InstanceDiscoveryDocumentCaptchaConfig,
    /// Instance-wide feature switches.
    pub features: InstanceDiscoveryDocumentFeatures,
    /// The GIF provider and its attribution rule.
    pub gif: InstanceDiscoveryDocumentGifProvider,
    /// Single sign-on state.
    pub sso: InstanceDiscoveryDocumentSSOStatus,
    /// Who can register.
    pub registration: InstanceDiscoveryDocumentRegistrationPolicy,
    /// Community and direct message policy.
    pub community: InstanceDiscoveryDocumentCommunityPolicy,
    /// Optional integrations that are available.
    pub services: InstanceDiscoveryDocumentServiceAvailability,
    /// Limit rules and trait names.
    pub limits: InstanceDiscoveryDocumentLimits,
    /// Web Push identity.
    pub push: InstanceDiscoveryDocumentPushNotificationConfig,
    /// Branding, setup state and legal links.
    pub app_public: InstanceDiscoveryDocumentAppPublic,
    /// Regional age policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_policy: Option<InstanceDiscoveryDocumentAgePolicy>,
    /// The current notice from the instance admins, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice: Option<InstanceDiscoveryDocumentNotice>,
    /// The first custom badges that guilds can show next to their name, in display order (at most 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_badges: Option<Vec<InstanceDiscoveryDocumentGuildBadge>>,
    /// True when the instance has more guild badges than `guild_badges` lists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_badges_partial: Option<bool>,
    /// The badge catalog version. It changes whenever a badge is created, edited, deleted or moved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_badges_version: Option<String>,
    /// The images the instance shows in place of the built-in guild badges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_in_guild_badge_images: Option<InstanceDiscoveryDocumentBuiltinGuildBadgeImages>,
    /// The built-in guild badges the instance doesn't show: any of verified, partnered and discoverable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_built_in_guild_badges: Option<Vec<InstanceDiscoveryDocumentBuiltinGuildBadge>>,
    /// The origin of every voice server endpoint, such as `wss://voice.example.com`,
    /// for content security policies (at most 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_origins: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use crate::client::HttpClient;

    use super::*;

    #[tokio::test]
    #[ignore = "requires reaching out to Fluxer's main instance"]
    async fn get_fluxer_main_instance_doc() {
        let client = HttpClient::builder().token(String::default()).build();

        let instance_doc = client
            .execute(InstanceDiscoveryDocument)
            .await
            .expect("should successfully parse the instance doc for the main Fluxer instance");

        dbg!(instance_doc);
    }
}
