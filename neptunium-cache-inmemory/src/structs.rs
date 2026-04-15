use std::{collections::HashMap, sync::Arc};

use atomic_once_cell::AtomicOnceCell;
use neptunium_http::endpoints::users::{MutualGuild, UserProfileFullResponse};
use neptunium_model::{
    channel::{
        Channel, ChannelType, PermissionOverwrite, VoiceRegion,
        message::{
            Message, MessageBase, MessageCall, MessageFlags, MessageReaction, MessageReference,
            MessageSnapshot, MessageSticker, MessageType, attachment::MessageAttachment,
            embed::MessageEmbed, nonce::Nonce,
        },
    },
    gateway::payload::incoming::UserPremiumType,
    guild::member::{GuildMember, GuildMemberProfile, GuildMemberProfileFlags},
    id::{
        AtomicId, Id,
        marker::{
            ChannelMarker, GuildMarker, MessageMarker, RoleMarker, UserMarker, WebhookMarker,
        },
    },
    misc::{HexColor32, ImageHash},
    time::timestamp::{Timestamp, representations::Iso8601},
    user::{PartialUser, UserExternalAccountConnection, UserProfileData},
};

use crate::{Cache, CacheValue, Cached, gateway::cached_payload::cache_option_vec};

#[derive(Clone, Debug)]
pub struct CachedChannel {
    /// The bitrate of the voice channel in bits per second
    pub bitrate: Option<i32>,
    /// `None` if this is a DM channel.
    pub guild_id: Option<Id<GuildMarker>>,
    /// The icon hash of the channel (for group DMs)
    pub icon: Option<String>,
    pub id: Id<ChannelMarker>,
    pub last_message_id: AtomicOnceCell<AtomicId<MessageMarker>>,
    pub last_pin_timestamp: Option<Timestamp<Iso8601>>,
    pub name: Option<String>,
    /// Custom nicknames for users in this channel (for group DMs)
    pub nicks: Option<HashMap<Id<UserMarker>, String>>,
    pub nsfw: Option<bool>,
    /// The ID of the owner of the channel (for group DMs)
    pub owner_id: Option<Id<UserMarker>>,
    pub parent_id: Option<Id<ChannelMarker>>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub position: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub recipients: Option<Vec<Cached<PartialUser>>>,
    pub rtc_region: Option<VoiceRegion>,
    pub topic: Option<String>,
    pub r#type: ChannelType,
    pub url: Option<String>,
    pub user_limit: Option<i32>,
}

impl CachedChannel {
    #[must_use]
    pub fn into_channel(self) -> Channel {
        let recipients = if let Some(cached_recipients) = self.recipients {
            let mut recipients = Vec::with_capacity(cached_recipients.len());
            for recipient in cached_recipients {
                recipients.push((*(*recipient.load())).clone());
            }
            Some(recipients)
        } else {
            None
        };

        Channel {
            bitrate: self.bitrate,
            guild_id: self.guild_id,
            icon: self.icon,
            id: self.id,
            last_message_id: self.last_message_id.into_inner().map(Into::into),
            last_pin_timestamp: self.last_pin_timestamp,
            name: self.name,
            nicks: self.nicks,
            nsfw: self.nsfw,
            owner_id: self.owner_id,
            parent_id: self.parent_id,
            permission_overwrites: self.permission_overwrites,
            position: self.position,
            rate_limit_per_user: self.rate_limit_per_user,
            recipients,
            rtc_region: self.rtc_region,
            topic: self.topic,
            r#type: self.r#type,
            url: self.url,
            user_limit: self.user_limit,
        }
    }

    #[must_use]
    pub fn from_channel(value: Channel, cache: &Arc<Cache>) -> Self {
        let recipients = cache_option_vec!(value.recipients, cache);
        Self {
            bitrate: value.bitrate,
            guild_id: value.guild_id,
            icon: value.icon,
            id: value.id,
            last_message_id: match value.last_message_id {
                Some(id) => AtomicOnceCell::from(AtomicId::from(id)),
                None => AtomicOnceCell::new(),
            },
            last_pin_timestamp: value.last_pin_timestamp,
            name: value.name,
            nicks: value.nicks,
            nsfw: value.nsfw,
            owner_id: value.owner_id,
            parent_id: value.parent_id,
            permission_overwrites: value.permission_overwrites,
            position: value.position,
            rate_limit_per_user: value.rate_limit_per_user,
            recipients,
            rtc_region: value.rtc_region,
            topic: value.topic,
            r#type: value.r#type,
            url: value.url,
            user_limit: value.user_limit,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CachedMessage {
    pub attachments: Option<Vec<MessageAttachment>>,
    pub author: Cached<PartialUser>,
    pub call: Option<MessageCall>,
    pub channel_id: Id<ChannelMarker>,
    pub content: String,
    pub edited_timestamp: Option<Timestamp<Iso8601>>,
    pub embeds: Option<Vec<MessageEmbed>>,
    pub flags: MessageFlags,
    pub id: Id<MessageMarker>,
    pub mention_everyone: bool,
    pub mention_roles: Option<Vec<Id<RoleMarker>>>,
    pub mentions: Option<Vec<Cached<PartialUser>>>,
    pub message_reference: Option<MessageReference>,
    /// Snapshots of forwarded messages.
    pub message_snapshots: Option<Vec<MessageSnapshot>>,
    /// A client-provided value for message deduplication.
    pub nonce: Option<Nonce>,
    pub pinned: bool,
    pub reactions: Option<Vec<MessageReaction>>,
    pub stickers: Option<Vec<MessageSticker>>,
    pub timestamp: Timestamp<Iso8601>,
    pub tts: bool,
    pub r#type: MessageType,
    pub webhook_id: Option<Id<WebhookMarker>>,
    /// The message that this message is replying to or forwarding.
    pub referenced_message: Option<CachedMessageBase>,
}

impl CachedMessage {
    #[must_use]
    pub fn from_message(value: Message, cache: &Arc<Cache>) -> Self {
        // To make rust happy, we need to destructure
        let Message {
            base:
                MessageBase {
                    attachments,
                    author,
                    call,
                    channel_id,
                    content,
                    edited_timestamp,
                    embeds,
                    flags,
                    id,
                    mention_everyone,
                    mention_roles,
                    mentions,
                    message_reference,
                    message_snapshots,
                    nonce,
                    pinned,
                    reactions,
                    stickers,
                    timestamp,
                    tts,
                    r#type,
                    webhook_id,
                },
            referenced_message,
        } = value;
        let author = author.insert_and_return(cache);
        let mentions = cache_option_vec!(mentions, cache);
        let referenced_message = referenced_message.map(|referenced_message| {
            CachedMessageBase::from_message_base(referenced_message, cache)
        });
        Self {
            attachments,
            author,
            call,
            channel_id,
            content,
            edited_timestamp,
            embeds,
            flags,
            id,
            mention_everyone,
            mention_roles,
            mentions,
            message_reference,
            message_snapshots,
            nonce,
            pinned,
            reactions,
            stickers,
            timestamp,
            tts,
            r#type,
            webhook_id,
            referenced_message,
        }
    }

    #[must_use]
    pub fn into_message(self) -> Message {
        let author = self.author.clone_inner();
        let mentions = if let Some(cached_mentions) = self.mentions {
            let mut mentions = Vec::with_capacity(cached_mentions.len());
            for mention in cached_mentions {
                mentions.push(mention.clone_inner());
            }
            Some(mentions)
        } else {
            None
        };
        let referenced_message = self.referenced_message.map(MessageBase::from);
        Message {
            base: MessageBase {
                attachments: self.attachments,
                author,
                call: self.call,
                channel_id: self.channel_id,
                content: self.content,
                edited_timestamp: self.edited_timestamp,
                embeds: self.embeds,
                flags: self.flags,
                id: self.id,
                mention_everyone: self.mention_everyone,
                mention_roles: self.mention_roles,
                mentions,
                message_reference: self.message_reference,
                message_snapshots: self.message_snapshots,
                nonce: self.nonce,
                pinned: self.pinned,
                reactions: self.reactions,
                stickers: self.stickers,
                timestamp: self.timestamp,
                tts: self.tts,
                r#type: self.r#type,
                webhook_id: self.webhook_id,
            },
            referenced_message,
        }
    }
}

impl From<CachedMessage> for Message {
    fn from(value: CachedMessage) -> Self {
        let author = value.author.clone_inner();
        let mentions = if let Some(cached_mentions) = value.mentions {
            let mut mentions = Vec::with_capacity(cached_mentions.len());
            for mention in cached_mentions {
                mentions.push(mention.clone_inner());
            }
            Some(mentions)
        } else {
            None
        };
        let referenced_message = value.referenced_message.map(MessageBase::from);
        Self {
            base: MessageBase {
                attachments: value.attachments,
                author,
                call: value.call,
                channel_id: value.channel_id,
                content: value.content,
                edited_timestamp: value.edited_timestamp,
                embeds: value.embeds,
                flags: value.flags,
                id: value.id,
                mention_everyone: value.mention_everyone,
                mention_roles: value.mention_roles,
                mentions,
                message_reference: value.message_reference,
                message_snapshots: value.message_snapshots,
                nonce: value.nonce,
                pinned: value.pinned,
                reactions: value.reactions,
                stickers: value.stickers,
                timestamp: value.timestamp,
                tts: value.tts,
                r#type: value.r#type,
                webhook_id: value.webhook_id,
            },
            referenced_message,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CachedMessageBase {
    pub attachments: Option<Vec<MessageAttachment>>,
    pub author: Cached<PartialUser>,
    pub call: Option<MessageCall>,
    pub channel_id: Id<ChannelMarker>,
    pub content: String,
    pub edited_timestamp: Option<Timestamp<Iso8601>>,
    pub embeds: Option<Vec<MessageEmbed>>,
    pub flags: MessageFlags,
    pub id: Id<MessageMarker>,
    pub mention_everyone: bool,
    pub mention_roles: Option<Vec<Id<RoleMarker>>>,
    pub mentions: Option<Vec<Cached<PartialUser>>>,
    pub message_reference: Option<MessageReference>,
    /// Snapshots of forwarded messages.
    pub message_snapshots: Option<Vec<MessageSnapshot>>,
    /// A client-provided value for message deduplication.
    pub nonce: Option<Nonce>,
    pub pinned: bool,
    pub reactions: Option<Vec<MessageReaction>>,
    pub stickers: Option<Vec<MessageSticker>>,
    pub timestamp: Timestamp<Iso8601>,
    pub tts: bool,
    pub r#type: MessageType,
    pub webhook_id: Option<Id<WebhookMarker>>,
}

impl CachedMessageBase {
    #[must_use]
    pub fn from_message_base(value: MessageBase, cache: &Arc<Cache>) -> Self {
        let author = value.author.insert_and_return(cache);
        let mentions = cache_option_vec!(value.mentions, cache);
        Self {
            attachments: value.attachments,
            author,
            call: value.call,
            channel_id: value.channel_id,
            content: value.content,
            edited_timestamp: value.edited_timestamp,
            embeds: value.embeds,
            flags: value.flags,
            id: value.id,
            mention_everyone: value.mention_everyone,
            mention_roles: value.mention_roles,
            mentions,
            message_reference: value.message_reference,
            message_snapshots: value.message_snapshots,
            nonce: value.nonce,
            pinned: value.pinned,
            reactions: value.reactions,
            stickers: value.stickers,
            timestamp: value.timestamp,
            tts: value.tts,
            r#type: value.r#type,
            webhook_id: value.webhook_id,
        }
    }
}

impl From<CachedMessageBase> for MessageBase {
    fn from(value: CachedMessageBase) -> Self {
        let author = value.author.clone_inner();
        let mentions = if let Some(cached_mentions) = value.mentions {
            let mut mentions = Vec::with_capacity(cached_mentions.len());
            for mention in cached_mentions {
                mentions.push(mention.clone_inner());
            }
            Some(mentions)
        } else {
            None
        };
        Self {
            attachments: value.attachments,
            author,
            call: value.call,
            channel_id: value.channel_id,
            content: value.content,
            edited_timestamp: value.edited_timestamp,
            embeds: value.embeds,
            flags: value.flags,
            id: value.id,
            mention_everyone: value.mention_everyone,
            mention_roles: value.mention_roles,
            mentions,
            message_reference: value.message_reference,
            message_snapshots: value.message_snapshots,
            nonce: value.nonce,
            pinned: value.pinned,
            reactions: value.reactions,
            stickers: value.stickers,
            timestamp: value.timestamp,
            tts: value.tts,
            r#type: value.r#type,
            webhook_id: value.webhook_id,
        }
    }
}

pub struct CachedGuildMember {
    pub accent_color: Option<HexColor32>,
    pub avatar: Option<ImageHash>,
    pub banner: Option<ImageHash>,
    /// Timestamp until which the member is timed out.
    pub communication_disabled_until: Option<Timestamp<Iso8601>>,
    pub deaf: bool,
    pub joined_at: Timestamp<Iso8601>,
    pub mute: bool,
    pub nick: Option<String>,
    pub profile_flags: Option<GuildMemberProfileFlags>,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: Cached<PartialUser>,
}

impl CachedGuildMember {
    pub fn from_guild_member(guild_member: GuildMember, cache: &Arc<Cache>) -> Self {
        let cached_user = guild_member.user.insert_and_return(cache);
        Self {
            accent_color: guild_member.accent_color,
            avatar: guild_member.avatar,
            banner: guild_member.banner,
            communication_disabled_until: guild_member.communication_disabled_until,
            deaf: guild_member.deaf,
            joined_at: guild_member.joined_at,
            mute: guild_member.mute,
            nick: guild_member.nick,
            profile_flags: guild_member.profile_flags,
            roles: guild_member.roles,
            user: cached_user,
        }
    }
}

impl From<CachedGuildMember> for GuildMember {
    fn from(value: CachedGuildMember) -> Self {
        let user = value.user.clone_inner();
        Self {
            accent_color: value.accent_color,
            avatar: value.avatar,
            banner: value.banner,
            communication_disabled_until: value.communication_disabled_until,
            deaf: value.deaf,
            joined_at: value.joined_at,
            mute: value.mute,
            nick: value.nick,
            profile_flags: value.profile_flags,
            roles: value.roles,
            user,
        }
    }
}

pub struct CachedUserProfileFullResponse {
    pub user: Cached<PartialUser>,
    pub user_profile: Cached<UserProfileData>,
    pub guild_member: Option<Cached<CachedGuildMember>>,
    pub guild_member_profile: Option<Cached<GuildMemberProfile>>,
    pub premium_type: Option<UserPremiumType>,
    pub premium_since: Option<Timestamp<Iso8601>>,
    /// Visionary ID.
    pub premium_lifetime_seqence: Option<u32>,
    pub mutual_friends: Option<Vec<Cached<PartialUser>>>,
    pub mutual_guilds: Option<Vec<MutualGuild>>,
    pub connected_accounts: Option<Vec<UserExternalAccountConnection>>,
}

impl CachedUserProfileFullResponse {
    pub fn from_user_profile_full_response(
        value: UserProfileFullResponse,
        guild_id: Option<Id<GuildMarker>>,
        cache: &Arc<Cache>,
    ) -> Self {
        let user_id = value.user.id;
        let user = value.user.insert_and_return(cache);
        let user_profile = (user_id, value.user_profile).insert_and_return(cache);
        let guild_member = if let Some(guild_id) = guild_id {
            value.guild_member.map(|member| {
                (
                    guild_id,
                    CachedGuildMember::from_guild_member(member, cache),
                )
                    .insert_and_return(cache)
            })
        } else {
            None
        };
        let guild_member_profile = if let Some(guild_id) = guild_id {
            value
                .guild_member_profile
                .map(|profile| (user_id, guild_id, profile).insert_and_return(cache))
        } else {
            None
        };
        let mutual_friends = cache_option_vec!(value.mutual_friends, cache);

        Self {
            user,
            user_profile,
            guild_member,
            guild_member_profile,
            premium_type: value.premium_type,
            premium_since: value.premium_since,
            premium_lifetime_seqence: value.premium_lifetime_seqence,
            mutual_friends,
            mutual_guilds: value.mutual_guilds,
            connected_accounts: value.connected_accounts,
        }
    }
}
