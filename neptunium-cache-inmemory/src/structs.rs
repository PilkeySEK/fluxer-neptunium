use std::{collections::HashMap, sync::Arc};

use atomic_once_cell::AtomicOnceCell;
use neptunium_http::endpoints::users::{MutualGuild, UserProfileFullResponse};
use neptunium_model::{
    channel::{
        Channel, ChannelType, PermissionOverwrite, VoiceRegion,
        message::{
            ChannelMention, Message, MessageCall, MessageFlags, MessageReaction, MessageReference,
            MessageSnapshot, MessageType, StickerItem, attachment::MessageAttachment, embed::Embed,
            nonce::Nonce,
        },
        thread::{ThreadChannel, ThreadMember, ThreadMemberFlags},
    },
    gateway::payload::incoming::{
        ThreadCreate, ThreadListSync, ThreadMemberListItem, ThreadMemberListItemPresence,
        ThreadMemberListUpdate, ThreadMemberUpdate, ThreadMembersUpdate, UserPremiumType,
    },
    guild::{
        member::{GuildMember, GuildMemberProfile, GuildMemberProfileFlags},
        permissions::{GuildRole, Permissions},
    },
    id::{
        AtomicId, Id,
        marker::{
            ChannelMarker, GuildMarker, MessageMarker, RoleMarker, UserMarker, WebhookMarker,
        },
    },
    misc::HexColor,
    time::timestamp::{Timestamp, representations::Iso8601},
    user::{
        PartialUser, UserExternalAccountConnection, UserProfileData,
        settings::GuildMuteConfiguration,
    },
};

use crate::{
    Cache, CacheValue, Cached,
    gateway::cached_payload::{cache_option_vec, cache_vec},
};

#[derive(Clone, Debug)]
pub struct CachedChannel {
    /// The bitrate of the voice channel in bits per second.
    pub bitrate: Option<i32>,
    /// `None` if this is a DM channel.
    pub guild_id: Option<Id<GuildMarker>>,
    /// The icon hash of the channel (for group DMs).
    pub icon: Option<String>,
    pub id: Id<ChannelMarker>,
    pub last_message_id: AtomicOnceCell<AtomicId<MessageMarker>>,
    pub last_pin_timestamp: Option<Timestamp<Iso8601>>,
    pub name: Option<String>,
    /// Custom nicknames for users in this channel (for group DMs).
    pub nicks: Option<HashMap<Id<UserMarker>, String>>,
    pub nsfw: Option<bool>,
    /// The ID of the owner of the channel (for group DMs).
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
    pub id: Id<MessageMarker>,
    pub channel_id: Id<ChannelMarker>,
    pub author: Cached<PartialUser>,
    pub webhook_id: Option<Id<WebhookMarker>>,
    pub r#type: MessageType,
    pub flags: MessageFlags,
    pub content: String,
    pub timestamp: Timestamp<Iso8601>,
    pub edited_timestamp: Option<Timestamp<Iso8601>>,
    pub pinned: bool,
    /// Whether the message mentions everyone.
    pub mention_everyone: bool,
    /// Whether the message requested text-to-speech.
    /// Echoed from the create request, so a message read through any other operation always reports false.
    pub tts: bool,
    pub mentions: Vec<Cached<PartialUser>>,
    pub mention_roles: Vec<Id<RoleMarker>>,
    // TODO: Maybe cache ChannelMention?
    pub mention_channels: Option<Vec<ChannelMention>>,
    /// Users referenced by non-notifying content, embed and snapshot text.
    pub users: Option<Vec<Cached<PartialUser>>>,
    pub embeds: Vec<Embed>,
    pub attachments: Vec<MessageAttachment>,
    pub stickers: Vec<StickerItem>,
    pub reactions: Option<Vec<MessageReaction>>,
    pub message_reference: Option<MessageReference>,
    pub message_snapshots: Option<Vec<MessageSnapshot>>,
    pub nonce: Option<Nonce>,
    pub call: Option<MessageCall>,
    pub referenced_message: Option<Cached<CachedMessage>>,
    pub thread: Option<ThreadChannel>,
}

impl CachedMessage {
    #[must_use]
    pub fn from_message(value: Message, cache: &Arc<Cache>) -> Self {
        // To make rust happy, we need to destructure
        let Message {
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
            mention_channels,
            users,
            thread,
        } = value;
        Self {
            attachments,
            author: author.insert_and_return(cache),
            call,
            channel_id,
            content,
            edited_timestamp,
            embeds,
            flags,
            id,
            mention_everyone,
            mention_roles,
            mentions: cache_vec!(mentions, cache),
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
            referenced_message: referenced_message.map(|referenced_message| {
                CachedMessage::from_message(*referenced_message, cache).insert_and_return(cache)
            }),
            mention_channels,
            users: cache_option_vec!(users, cache),
            thread,
        }
    }
}

impl From<CachedMessage> for Message {
    fn from(value: CachedMessage) -> Self {
        let author = value.author.clone_inner();
        Message {
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
            mentions: value
                .mentions
                .into_iter()
                .map(|mention| mention.clone_inner())
                .collect(),
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
            referenced_message: value
                .referenced_message
                .map(|message| Box::new(message.clone_inner().into())),
            mention_channels: value.mention_channels,
            users: value
                .users
                .map(|users| users.into_iter().map(|user| user.clone_inner()).collect()),
            thread: value.thread,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedGuildMember {
    pub accent_color: Option<HexColor>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    /// Timestamp until which the member is timed out.
    pub communication_disabled_until: Option<Timestamp<Iso8601>>,
    pub deaf: bool,
    pub joined_at: Timestamp<Iso8601>,
    pub mute: bool,
    pub nick: Option<String>,
    pub profile_flags: Option<GuildMemberProfileFlags>,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: Cached<PartialUser>,
    pub guild_id: Id<GuildMarker>,
    pub id: Id<UserMarker>,
}

impl CachedGuildMember {
    pub fn from_guild_member(
        guild_member: GuildMember,
        guild_id: Id<GuildMarker>,
        cache: &Arc<Cache>,
    ) -> Self {
        let cached_user = guild_member.user.insert_and_return(cache);
        let id = cached_user.load().id;
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
            guild_id,
            id,
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
                CachedGuildMember::from_guild_member(member, guild_id, cache)
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

impl From<&CachedChannel> for Id<ChannelMarker> {
    fn from(value: &CachedChannel) -> Self {
        value.id
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedGuildRole {
    pub id: Id<RoleMarker>,
    pub name: String,
    pub permissions: Permissions,
    pub position: u16,
    pub color: HexColor,
    /// Hash of the icon.
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub hoist: bool,
    pub hoist_position: Option<u16>,
    pub mentionable: bool,
    pub guild_id: Id<GuildMarker>,
}

impl CachedGuildRole {
    #[must_use]
    pub fn from_guild_role(value: GuildRole, guild_id: Id<GuildMarker>) -> Self {
        Self {
            id: value.id,
            name: value.name,
            permissions: value.permissions,
            position: value.position,
            color: value.color,
            icon: value.icon,
            unicode_emoji: value.unicode_emoji,
            hoist: value.hoist,
            hoist_position: value.hoist_position,
            mentionable: value.mentionable,
            guild_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedThreadMember {
    /// The ID of the thread.
    /// Omitted on the member of a thread in `Ready`, `GuildCreate`, and `GuildSync`.
    pub thread_id: Option<Id<ChannelMarker>>,
    /// Omitted on the member of a thread in `Ready`, `GuildCreate`, and `GuildSync`.
    pub user_id: Option<Id<UserMarker>>,
    /// The time the user last joined the thread.
    pub join_timestamp: Timestamp<Iso8601>,
    pub flags: ThreadMemberFlags,
    /// Whether the user muted the thread.
    /// Present only on the caller's own membership.
    pub muted: Option<bool>,
    ///  In an HTTP response, present only when the request sets `with_member` and the user is still a guild member.
    /// `ThreadMembersUpdate` defines it on each `added_members` entry.
    pub mute_config: Option<GuildMuteConfiguration>,
    ///  In an HTTP response, present only when the request sets `with_member` and the user is still a guild member.
    /// `ThreadMembersUpdate` defines it on each `added_members` entry.
    pub guild_member: Option<Cached<CachedGuildMember>>,
}

impl From<(ThreadMember, Id<GuildMarker>, &Arc<Cache>)> for CachedThreadMember {
    fn from((member, guild_id, cache): (ThreadMember, Id<GuildMarker>, &Arc<Cache>)) -> Self {
        Self {
            thread_id: member.thread_id,
            user_id: member.user_id,
            join_timestamp: member.join_timestamp,
            flags: member.flags,
            muted: member.muted,
            mute_config: member.mute_config,
            guild_member: member.guild_member.map(|member| {
                CachedGuildMember::from_guild_member(member, guild_id, cache)
                    .insert_and_return(cache)
            }),
        }
    }
}

impl From<CachedThreadMember> for ThreadMember {
    fn from(value: CachedThreadMember) -> Self {
        Self {
            thread_id: value.thread_id,
            user_id: value.user_id,
            join_timestamp: value.join_timestamp,
            flags: value.flags,
            muted: value.muted,
            mute_config: value.mute_config,
            guild_member: value.guild_member.map(|member| member.clone_inner().into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedThreadCreate {
    pub thread: Cached<ThreadChannel>,
    /// `Some(true)` when the thread was just created, `None` when the session was added to an existing thread.
    pub newly_created: Option<bool>,
    /// The session's own membership.
    pub member: Option<CachedThreadMember>,
}

impl From<(ThreadCreate, &Arc<Cache>)> for CachedThreadCreate {
    fn from((event, cache): (ThreadCreate, &Arc<Cache>)) -> Self {
        Self {
            member: event
                .member
                .map(|member| (member, event.thread.guild_id, cache).into()),
            thread: event.thread.insert_and_return(cache),
            newly_created: event.newly_created,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedThreadListSync {
    pub guild_id: Id<GuildMarker>,
    /// `None` for the whole guild.
    pub channel_ids: Option<Vec<Id<ChannelMarker>>>,
    pub threads: Vec<Cached<ThreadChannel>>,
    pub members: Vec<CachedThreadMember>,
}

impl From<(ThreadListSync, &Arc<Cache>)> for CachedThreadListSync {
    fn from((event, cache): (ThreadListSync, &Arc<Cache>)) -> Self {
        Self {
            guild_id: event.guild_id,
            channel_ids: event.channel_ids,
            threads: event
                .threads
                .into_iter()
                .map(|thread| thread.insert_and_return(cache))
                .collect(),
            members: event
                .members
                .into_iter()
                .map(|member| (member, event.guild_id, cache).into())
                .collect(),
        }
    }
}

impl From<CachedThreadListSync> for ThreadListSync {
    fn from(value: CachedThreadListSync) -> Self {
        Self {
            guild_id: value.guild_id,
            channel_ids: value.channel_ids,
            threads: value
                .threads
                .into_iter()
                .map(|thread| thread.clone_inner())
                .collect(),
            members: value
                .members
                .into_iter()
                .map(|member| member.into())
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedThreadMemberUpdate {
    pub thread_member: CachedThreadMember,
    pub guild_id: Id<GuildMarker>,
}

impl From<(ThreadMemberUpdate, &Arc<Cache>)> for CachedThreadMemberUpdate {
    fn from((event, cache): (ThreadMemberUpdate, &Arc<Cache>)) -> Self {
        Self {
            thread_member: (event.thread_member, event.guild_id, cache).into(),
            guild_id: event.guild_id,
        }
    }
}

impl From<CachedThreadMemberUpdate> for ThreadMemberUpdate {
    fn from(value: CachedThreadMemberUpdate) -> Self {
        Self {
            thread_member: value.thread_member.into(),
            guild_id: value.guild_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedThreadMembersUpdate {
    /// The thread ID.
    pub id: Id<ChannelMarker>,
    pub guild_id: Id<GuildMarker>,
    /// The number of thread members, max. `50`.
    pub member_count: usize,
    // TODO: https://web.fluxer.app/channels/1484905952721252427/1484905952721252430/1563642795653070848
    pub added_members: Option<Vec<CachedThreadMember>>,
    pub removed_member_ids: Option<Vec<Id<UserMarker>>>,
}

impl From<(ThreadMembersUpdate, &Arc<Cache>)> for CachedThreadMembersUpdate {
    fn from((event, cache): (ThreadMembersUpdate, &Arc<Cache>)) -> Self {
        Self {
            id: event.id,
            guild_id: event.guild_id,
            member_count: event.member_count,
            added_members: event.added_members.map(|added_members| {
                added_members
                    .into_iter()
                    .map(|member| (member, event.guild_id, cache).into())
                    .collect()
            }),
            removed_member_ids: event.removed_member_ids,
        }
    }
}

impl From<CachedThreadMembersUpdate> for ThreadMembersUpdate {
    fn from(value: CachedThreadMembersUpdate) -> Self {
        Self {
            id: value.id,
            guild_id: value.guild_id,
            member_count: value.member_count,
            added_members: value.added_members.map(|added_members| {
                added_members
                    .into_iter()
                    .map(|member| member.into())
                    .collect()
            }),
            removed_member_ids: value.removed_member_ids,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedThreadMemberListUpdate {
    pub guild_id: Id<GuildMarker>,
    pub thread_id: Id<ChannelMarker>,
    pub members: Vec<CachedThreadMemberListItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedThreadMemberListItem {
    pub user_id: Id<UserMarker>,
    pub join_timestamp: Option<Timestamp<Iso8601>>,
    /// Undocumented (?)
    pub flags: u64,
    pub member: Option<Cached<CachedGuildMember>>,
    pub presence: Option<ThreadMemberListItemPresence>,
}

impl From<(ThreadMemberListUpdate, &Arc<Cache>)> for CachedThreadMemberListUpdate {
    fn from((event, cache): (ThreadMemberListUpdate, &Arc<Cache>)) -> Self {
        Self {
            guild_id: event.guild_id,
            thread_id: event.thread_id,
            members: event
                .members
                .into_iter()
                .map(|item| (item, event.guild_id, cache).into())
                .collect(),
        }
    }
}

impl From<(ThreadMemberListItem, Id<GuildMarker>, &Arc<Cache>)> for CachedThreadMemberListItem {
    fn from((item, guild_id, cache): (ThreadMemberListItem, Id<GuildMarker>, &Arc<Cache>)) -> Self {
        Self {
            user_id: item.user_id,
            join_timestamp: item.join_timestamp,
            flags: item.flags,
            member: item.member.map(|member| {
                CachedGuildMember::from_guild_member(member, guild_id, cache)
                    .insert_and_return(cache)
            }),
            presence: item.presence,
        }
    }
}
