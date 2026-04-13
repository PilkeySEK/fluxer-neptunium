use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::RoleMarker},
    misc::{HexColor32, ImageHash},
    serde_bitflags,
};

bitflags! {
    /// Permissions are serialized and deserialized as a String.
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct Permissions: u64 {
        /// Allows creation of instant invites.
        const CREATE_INSTANT_INVITE = 1 << 0;
        /// Allows kicking members from the guild.
        const KICK_MEMBERS = 1 << 1;
        /// Allows banning members from the guild.
        const BAN_MEMBERS = 1 << 2;
        /// Grants all permissions and bypasses channel permission overwrites.
        const ADMINISTRATOR = 1 << 3;
        /// Allows management and editing of channels.
        const MANAGE_CHANNELS = 1 << 4;
        /// Allows management and editing of the guild.
        const MANAGE_GUILD = 1 << 5;
        /// Allows adding reactions to messages.
        const ADD_REACTIONS = 1 << 6;
        /// Allows viewing of the audit log.
        const VIEW_AUDIT_LOG = 1 << 7;
        /// Allows using priority speaker in a voice channel.
        const PRIORITY_SPEAKER = 1 << 8;
        /// Allows the user to go live.
        const STREAM = 1 << 9;
        /// Allows viewing a channel.
        const VIEW_CHANNEL = 1 << 10;
        /// Allows sending messages in a channel.
        const SEND_MESSAGES = 1 << 11;
        /// Allows sending text-to-speech messages.
        const SEND_TTS_MESSAGES = 1 << 12;
        /// Allows for deleting and pinning messages.
        const MANAGE_MESSAGES = 1 << 13;
        /// Links sent will have an embed automatically.
        const EMBED_LINKS = 1 << 14;
        /// Allows uploading files.
        const ATTACH_FILES = 1 << 15;
        /// Allows reading message history.
        const READ_MESSAGE_HISTORY = 1 << 16;
        /// Allows using @everyone and @here mentions.
        const MENTION_EVERYONE = 1 << 17;
        /// Allows using emojis from other guilds.
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        /// Allows connecting to a voice channel.
        const CONNECT = 1 << 20;
        /// Allows speaking in a voice channel.
        const SPEAK = 1 << 21;
        /// Allows muting members in voice channels.
        const MUTE_MEMBERS = 1 << 22;
        /// Allows deafening members in voice channels.
        const DEAFEN_MEMBERS = 1 << 23;
        /// Allows moving members between voice channels.
        const MOVE_MEMBERS = 1 << 24;
        /// Allows using voice activity detection.
        const USE_VAD = 1 << 25;
        /// Allows changing own nickname.
        const CHANGE_NICKNAME = 1 << 26;
        /// Allows changing other members nicknames.
        const MANAGE_NICKNAMES = 1 << 27;
        /// Allows management and editing of roles.
        const MANAGE_ROLES = 1 << 28;
        /// Allows management and editing of webhooks.
        const MANAGE_WEBHOOKS = 1 << 29;
        /// Allows management of guild expressions.
        const MANAGE_EXPRESSIONS = 1 << 30;
        /// Allows using stickers from other guilds.
        const USE_EXTERNAL_STICKERS = 1 << 37;
        /// Allows timing out users.
        const MODERATE_MEMBERS = 1 << 40;
        /// Allows creating guild expressions.
        const CREATE_EXPRESSIONS = 1 << 43;
        /// Allows pinning messages.
        const PIN_MESSAGES = 1 << 51;
        /// Allows bypassing slowmode.
        const BYPASS_SLOWMODE = 1 << 52;
        /// Allows updating the voice region.
        const UPDATE_RTC_REGION = 1 << 53;
    }
}

serde_bitflags! {Permissions, String(u64)}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GuildRole {
    pub id: Id<RoleMarker>,
    pub name: String,
    pub permissions: Permissions,
    pub position: u16,
    pub color: HexColor32,
    /// Hash of the icon
    pub icon: Option<ImageHash>,
    pub unicode_emoji: Option<String>,
    pub hoist: bool,
    pub hoist_position: Option<u16>,
    pub mentionable: bool,
}
