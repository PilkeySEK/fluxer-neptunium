use serde::Deserialize;

use crate::{
    channel::Channel,
    gateway::{presence::Presence, voice_state::VoiceState},
    guild::{
        Guild,
        member::GuildMember,
        properties::{
            DefaultMessageNotifications, GuildEmoji, GuildExplicitContentFilter, GuildFeatureFlag,
            GuildMfaLevel, GuildOperations, GuildSticker, GuildVerificationLevel, NsfwLevel,
            SplashCardAlignment, SystemChannelFlags,
        },
    },
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
    misc::ImageHash,
    time::{
        duration::{Duration, representation::Seconds},
        timestamp::{Timestamp, representations::Iso8601},
    },
};

#[derive(Deserialize, Clone, Debug)]
pub struct GuildProperties {
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: Duration<Seconds>,
    pub banner: Option<ImageHash>,
    pub banner_height: Option<u32>,
    pub banner_width: Option<u32>,
    pub default_message_notifications: DefaultMessageNotifications,
    pub disabled_operations: GuildOperations,
    /// Base64-encoded image data for the embedded invite splash.
    pub embed_splash: Option<ImageHash>,
    pub embed_splash_height: Option<u32>,
    pub embed_splash_width: Option<u32>,
    pub explicit_content_filter: GuildExplicitContentFilter,
    pub features: Vec<GuildFeatureFlag>,
    /// Hash of the guild icon
    pub icon: Option<ImageHash>,
    pub id: Id<GuildMarker>,
    pub message_history_cutoff: Option<Timestamp<Iso8601>>,
    pub mfa_level: GuildMfaLevel,
    pub name: String,
    pub nsfw_level: NsfwLevel,
    pub owner_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    /// Base64-encoded image data for the guild splash screen.
    pub splash: Option<ImageHash>,
    pub splash_card_alignment: SplashCardAlignment,
    pub splash_height: Option<u32>,
    pub splash_width: Option<u32>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub vanity_url_code: Option<String>,
    pub verification_level: GuildVerificationLevel,
}

// Figured out by looking at guild response... :(
// TODO: Check official gateway code once fluxer-v2 is released
#[derive(Deserialize, Clone, Debug)]
pub struct GuildCreate {
    pub properties: GuildProperties,
    pub channels: Vec<Channel>,
    pub id: Id<GuildMarker>,
    pub member_count: u64,
    pub online_count: u64,
    pub stickers: Vec<GuildSticker>,
    pub emojis: Vec<GuildEmoji>,
    pub members: Vec<GuildMember>,
    pub presences: Vec<Presence>,
    pub voice_states: Vec<VoiceState>,
    pub joined_at: Timestamp<Iso8601>,
}

impl From<GuildProperties> for Guild {
    fn from(value: GuildProperties) -> Self {
        Self {
            afk_channel_id: value.afk_channel_id,
            afk_timeout: value.afk_timeout,
            banner: value.banner,
            banner_width: value.banner_width,
            banner_height: value.banner_height,
            current_user_permissions: None,
            default_message_notifications: value.default_message_notifications,
            disabled_operations: value.disabled_operations,
            embed_splash: value.embed_splash,
            embed_splash_height: value.embed_splash_height,
            embed_splash_width: value.embed_splash_width,
            explicit_content_filter: value.explicit_content_filter,
            features: value.features,
            icon: value.icon,
            id: value.id,
            message_history_cutoff: value.message_history_cutoff,
            mfa_level: value.mfa_level,
            name: value.name,
            nsfw_level: value.nsfw_level,
            owner_id: value.owner_id,
            rules_channel_id: value.rules_channel_id,
            splash: value.splash,
            splash_card_alignment: value.splash_card_alignment,
            splash_width: value.splash_width,
            splash_height: value.splash_height,
            system_channel_flags: value.system_channel_flags,
            system_channel_id: value.system_channel_id,
            vanity_url_code: value.vanity_url_code,
            verification_level: value.verification_level,
        }
    }
}
