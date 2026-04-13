use bitflags::bitflags;

use crate::serde_bitflags;

// TODO: Find out which of these actually exist in Fluxer
// Source of these bitflags: https://docs.discord.com/developers/events/gateway#gateway-intents
bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct GatewayEventFlags: u64 {
        /// Correlates to the following events:
        /// - `GUILD_CREATE`
        /// - `GUILD_UPDATE`
        /// - `GUILD_DELETE`
        /// - `GUILD_ROLE_CREATE`
        /// - `GUILD_ROLE_UPDATE`
        /// - `GUILD_ROLE_DELETE`
        /// - `CHANNEL_CREATE`
        /// - `CHANNEL_UPDATE`
        /// - `CHANNEL_DELETE`
        /// - `CHANNEL_PINS_UPDATE`
        /// - `THREAD_CREATE`
        /// - `THREAD_UPDATE`
        /// - `THREAD_DELETE`
        /// - `THREAD_LIST_SYNC`
        /// - `THREAD_MEMBER_UPDATE`
        /// - `THREAD_MEMBERS_UPDATE`
        /// - `STAGE_INSTANCE_CREATE`
        /// - `STAGE_INSTANCE_UPDATE`
        /// - `STAGE_INSTANCE_DELETE`
        const GUILDS = 1 << 0;
        /// Correlates to the following events:
        /// - `GUILD_MEMBER_ADD`
        /// - `GUILD_MEMBER_UPDATE`
        /// - `GUILD_MEMBER_REMOVE`
        /// - `THREAD_MEMBERS_UPDATE`
        const GUILD_MEMBERS = 1 << 1;
        /// Correlates to the following events:
        /// - `GUILD_AUDIT_LOG_ENTRY_CREATE`
        /// - `GUILD_BAN_ADD`
        /// - `GUILD_BAN_REMOVE`
        const GUILD_MODERATION = 1 << 2;
        /// Correlates to the following events:
        /// - `GUILD_EMOJIS_UPDATE`
        /// - `GUILD_STICKERS_UPDATE`
        /// - `GUILD_SOUNDBOARD_SOUND_CREATE`
        /// - `GUILD_SOUNDBOARD_SOUND_UPDATE`
        /// - `GUILD_SOUNDBOARD_SOUND_DELETE`
        /// - `GUILD_SOUNDBOARD_SOUNDS_UPDATE`
        const GUILD_EXPRESSIONS = 1 << 3;
        /// Correlates to the following events:
        /// - `GUILD_INTEGRATIONS_UPDATE`
        /// - `INTEGRATION_CREATE`
        /// - `INTEGRATION_UPDATE`
        /// - `INTEGRATION_DELETE`
        const GUILD_INTEGRATIONS = 1 << 4;
        /// Correlates to the following events:
        /// - `WEBHOOKS_UPDATE`
        const GUILD_WEBHOOKS = 1 << 5;
        /// Correlates to the following events:
        /// - `INVITE_CREATE`
        /// - `INVITE_DELETE`
        const GUILD_INVITES = 1 << 6;
        /// Correlates to the following events:
        /// - `VOICE_CHANNEL_EFFECT_SEND`
        /// - `VOICE_STATE_UPDATE`
        const GUILD_VOICE_STATES = 1 << 7;
        /// Correlates to the following events:
        /// - `PRESENCE_UPDATE`
        const GUILD_PRESENCES = 1 << 8;
        /// Correlates to the following events:
        /// - `MESSAGE_CREATE`
        /// - `MESSAGE_UPDATE`
        /// - `MESSAGE_DELETE`
        /// - `MESSAGE_DELETE_BULK`
        const GUILD_MESSAGES = 1 << 9;
        /// Correlates to the following events:
        /// - `MESSAGE_REACTION_ADD`
        /// - `MESSAGE_REACTION_REMOVE`
        /// - `MESSAGE_REACTION_REMOVE_ALL`
        /// - `MESSAGE_REACTION_REMOVE_EMOJI`
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        /// Correlates to the following events:
        /// - `TYPING_START`
        const GUILD_MESSAGE_TYPING = 1 << 11;
        /// Correlates to the following events:
        /// - `MESSAGE_CREATE`
        /// - `MESSAGE_UPDATE`
        /// - `MESSAGE_DELETE`
        /// - `CHANNEL_PINS_UPDATE`
        const DIRECT_MESSAGES = 1 << 12;
        /// Correlates to the following events:
        /// - `MESSAGE_REACTION_ADD`
        /// - `MESSAGE_REACTION_REMOVE`
        /// - `MESSAGE_REACTION_REMOVE_ALL`
        /// - `MESSAGE_REACTION_REMOVE_EMOJI`
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        /// Correlates to the following events:
        /// - `TYPING_START`
        const DIRECT_MESSAGE_TYPING = 1 << 14;
        /// This does not represent individual events, but rather affects what data is
        /// present for events that could contain content fields.
        const MESSAGE_CONTENT = 1 << 15;
        /// Correlates to the following events:
        /// - `GUILD_SCHEDULED_EVENT_CREATE`
        /// - `GUILD_SCHEDULED_EVENT_UPDATE`
        /// - `GUILD_SCHEDULED_EVENT_DELETE`
        /// - `GUILD_SCHEDULED_EVENT_USER_ADD`
        /// - `GUILD_SCHEDULED_EVENT_USER_REMOVE`
        const GUILD_SCHEDULED_EVENTS = 1 << 16;
        /// Correlates to the following events:
        /// - `AUTO_MODERATION_RULE_CREATE`
        /// - `AUTO_MODERATION_RULE_UPDATE`
        /// - `AUTO_MODERATION_RULE_DELETE`
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;
        /// Correlates to the following events:
        /// - `AUTO_MODERATION_ACTION_EXECUTION`
        const AUTO_MODERATION_EXECUTION = 1 << 21;
        /// Correlates to the following events:
        /// - `MESSAGE_POLL_VOTE_ADD`
        /// - `MESSAGE_POLL_VOTE_REMOVE`
        const GUILD_MESSAGE_POLLS = 1 << 24;
        /// Correlates to the following events:
        /// - `MESSAGE_POLL_VOTE_ADD`
        /// - `MESSAGE_POLL_VOTE_REMOVE`
        const DIRECT_MESSAGE_POLLS = 1 << 25;
    }
}

// I am not covering this API so IDK, but is this meant to be as a string?
/* serde_bitflags! {GatewayEventFlags, String(u64)} */
serde_bitflags! {GatewayEventFlags, u64}
