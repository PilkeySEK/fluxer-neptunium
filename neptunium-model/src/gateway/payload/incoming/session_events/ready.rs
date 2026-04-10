use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use zeroize::Zeroizing;

use crate::{
    channel::Channel,
    gateway::presence::Presence,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, RelationshipMarker, UserMarker},
    },
    time::timestamp::{Timestamp, representations::Iso8601},
    user::{
        PartialUser,
        flags::PublicUserFlags,
        read_state::ReadState,
        relationship::RelationshipType,
        settings::{FavoriteMeme, UserGuildSettings, UserSettings},
    },
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GuildReadyResponse {
    pub id: Id<GuildMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<Timestamp<Iso8601>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivateResponsePendingBulkMessageDeletion {
    pub channel_count: i32,
    pub message_count: i32,
    pub scheduled_at: Timestamp<Iso8601>,
}

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UserAuthenticatorType {
    TimeBasedOneTimePassword = 0,
    SMSBased = 1,
    WebAuthn = 2,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivateResponse {
    pub accent_color: Option<i32>,
    /// Access control list entries for the user
    pub acls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_types: Option<Vec<UserAuthenticatorType>>,
    pub avatar: Option<String>,
    pub avatar_color: Option<i32>,
    pub banner: Option<String>,
    pub banner_color: Option<i32>,
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    pub discriminator: String,
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_bounced: Option<bool>,
    pub flags: PublicUserFlags,
    pub global_name: Option<String>,
    pub has_dismissed_premium_onboarding: bool,
    pub has_ever_purchased: bool,
    pub has_unread_gift_inventory: bool,
    pub id: Id<UserMarker>,
    pub is_staff: bool,
    pub mfa_enabled: bool,
    pub nsfw_allowed: bool,
    pub password_last_changed_at: Option<Timestamp<Iso8601>>,
    pub pending_bulk_message_deletion: Option<UserPrivateResponsePendingBulkMessageDeletion>,
    pub phone: Option<String>,
    pub premium_badge_hidden: bool,
    pub premium_badge_masked: bool,
    pub premium_badge_sequence_hidden: bool,
    pub premium_badge_timestamp_hidden: bool,
    pub premium_billing_cylcle: Option<String>,
    pub premium_enabled_override: bool,
    pub premium_lifetime_sequence: Option<i32>,
    pub premium_purchase_disabled: bool,
    pub premium_since: Option<Timestamp<Iso8601>>,
    pub premium_type: Option<UserPremiumType>,
    pub premium_will_cancel: bool,
    pub pronouns: Option<String>,
    pub required_actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    pub traits: Vec<String>,
    pub unread_gift_inventory_count: u32,
    pub used_mobile_client: bool,
    pub username: String,
    /// Whether the email address has been verified
    pub verified: bool,
    // Undocumented.
    // TODO: Is this optional?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "user_api")]
    pub privacy_agreed_at: Option<Timestamp<Iso8601>>,
    // Undocumented.
    // TODO: Is this optional?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "user_api")]
    pub terms_agreed_at: Option<Timestamp<Iso8601>>,
    // Undocumented.
    #[cfg(feature = "user_api")]
    pub premium_out_of_band_trial_ends_at: Option<Timestamp<Iso8601>>,
}

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UserPremiumType {
    /// No premium subscription.
    None = 0,
    Active,
    Lifetime,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct RtcRegion {
    pub emoji: String,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationshipReadyResponse {
    pub id: Id<RelationshipMarker>,
    /// A custom nickname set for the related user.
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<Timestamp<Iso8601>>,
    #[serde(rename = "type")]
    pub r#type: RelationshipType,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Ready {
    pub version: u64,
    pub session_id: Zeroizing<String>,
    pub user: UserPrivateResponse,
    pub guilds: Vec<GuildReadyResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_channels: Option<Vec<Channel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<RelationshipReadyResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<PartialUser>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presences: Option<Vec<Presence>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<UserSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_guild_settings: Option<Vec<UserGuildSettings>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_states: Option<Vec<ReadState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<HashMap<Id<UserMarker>, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_dms: Option<Vec<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite_memes: Option<Vec<FavoriteMeme>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_session_id_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_regions: Option<Vec<RtcRegion>>,
    // TODO: Check whether latitude and longitude are actually optional
    #[cfg(feature = "user_api")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[cfg(feature = "user_api")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
}
