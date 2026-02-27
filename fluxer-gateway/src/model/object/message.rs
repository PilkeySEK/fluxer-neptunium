use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::OffsetDateTime;

use crate::model::{object::user::UserPartialResponse, snowflake::Snowflake};

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    /// Deletable: `true`
    Default = 0,
    /// Deletable: `false`
    RecipientAdd = 1,
    /// Deletable: `false`
    RecipientRemove = 2,
    /// Deletable: `false`
    Call = 3,
    /// Deletable: `false`
    ChannelNameChange = 4,
    /// Deletable: `false`
    ChannelIconChange = 5,
    /// Deletable: `true`
    ChannelPinnedMessage = 6,
    /// Deletable: `true`
    UserJoin = 7,
    /// Deletable: `true`
    GuildBoost = 8,
    /// Deletable: `true`
    GuildBoostTier1 = 9,
    /// Deletable: `true`
    GuildBoostTier2 = 10,
    /// Deletable: `true`
    GuildBoostTier3 = 11,
    /// Deletable: `true`
    ChannelFollowAdd = 12,
    /// Deletable: `true`
    GuildDiscoveryDisqualified = 14,
    /// Deletable: `true`
    GuildDiscoveryRequalified = 15,
    /// Deletable: `true`
    GuildDiscoveryGracePeriodInitialWarning = 16,
    /// Deletable: `true`
    GuildDiscoveryGracePeriodFinalWarning = 17,
    /// Deletable: `true`
    ThreadCreated = 18,
    /// Deletable: `true`
    Reply = 19,
    /// Deletable: `true`
    ChatInputCommand = 20,
    /// Deletable: `false`
    ThreadStarterMessage = 21,
    /// Deletable: `true`
    GuildInviteReminder = 22,
    /// Deletable: `true`
    ContextMenuCommand = 23,
    /// Deletable: `true` (only deletable by members with `MANAGE_MESSAGES` permission)
    AutoModerationAction = 24,
    /// Deletable: `true`
    RoleSubscriptionPurchase = 25,
    /// Deletable: `true`
    InteractionPremiumUpsell = 26,
    /// Deletable: `true`
    StageStart = 27,
    /// Deletable: `true`
    StageEnd = 28,
    /// Deletable: `true`
    StageSpeaker = 29,
    /// Deletable: `true`
    StageTopic = 31,
    /// Deletable: `true`
    GuildApplicationPremiumSubscription = 32,
    /// Deletable: `true`
    GuildIncidentAlertModeEnabled = 36,
    /// Deletable: `true`
    GuildIncidentAlertModeDisabled = 37,
    /// Deletable: `true`
    GuildIncidentReportRaid = 38,
    /// Deletable: `true`
    GuildIncidentReportFalseAlarm = 39,
    /// Deletable: `true`
    PurchaseNotification = 44,
    /// Deletable: `true`
    PollResult = 46,
}

// Not officially documented?
// https://docs.fluxer.app/api-reference/channels/send-a-message ?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub author: UserPartialResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub r#type: MessageType,
    pub flags: i32,
    pub content: String,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub pinned: bool,
    pub mention_everyone: bool,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::iso8601::option"
    )]
    pub edited_timestamp: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<UserPartialResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_roles: Option<Vec<Snowflake>>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<serde_json::Value>>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<serde_json::Value>>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<serde_json::Value>>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<serde_json::Value>>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<serde_json::Value>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_snapshots: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call: Option<serde_json::Value>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<serde_json::Value>,
}
