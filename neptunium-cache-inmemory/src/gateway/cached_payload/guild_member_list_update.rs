use neptunium_model::{
    gateway::payload::incoming::{
        GuildMemberListUpdate, MemberListGroup, MemberListOperationType, MemberListPresence,
    },
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
};

use crate::{CacheValue, Cached, CachedGuildMember, gateway::cached_payload::CachedPayload};

pub struct CachedGuildMemberListUpdate {
    pub guild_id: Id<GuildMarker>,
    /// The channel for which the member list is updated.
    pub id: Id<ChannelMarker>,
    /// Same as `id`.
    pub channel_id: Option<Id<ChannelMarker>>,
    pub member_count: usize,
    pub online_count: usize,
    pub groups: Vec<MemberListGroup>,
    pub ops: Vec<CachedMemberListOperation>,
}

impl CachedPayload for CachedGuildMemberListUpdate {
    type NonCached = GuildMemberListUpdate;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        let guild_id = non_cached.guild_id;
        Self {
            guild_id: non_cached.guild_id,
            id: non_cached.id,
            channel_id: non_cached.channel_id,
            member_count: non_cached.member_count,
            online_count: non_cached.online_count,
            groups: non_cached.groups,
            ops: non_cached
                .ops
                .into_iter()
                .map(|operation| CachedMemberListOperation {
                    op: operation.op,
                    range: operation.range,
                    items: operation.items.map(|items| {
                        items
                            .into_iter()
                            .map(|item| CachedMemberListItem {
                                member: item.member.map(|member| {
                                    CachedGuildMember::from_guild_member(member, guild_id, cache)
                                        .insert_and_return(cache)
                                }),
                                group: item.group,
                            })
                            .collect()
                    }),
                    index: operation.index,
                    item: operation.item.map(|item| CachedMemberListItem {
                        member: item.member.map(|member| {
                            CachedGuildMember::from_guild_member(member, guild_id, cache)
                                .insert_and_return(cache)
                        }),
                        group: item.group,
                    }),
                })
                .collect(),
        }
    }
}

// TODO: Find out which of these are sent for which ops
#[derive(Clone, Debug)]
pub struct CachedMemberListOperation {
    pub op: MemberListOperationType,
    pub range: Option<(usize, usize)>,
    pub items: Option<Vec<CachedMemberListItem>>,
    pub index: Option<usize>,
    pub item: Option<CachedMemberListItem>,
}

#[derive(Clone, Debug)]
pub struct CachedMemberListItem {
    pub member: Option<Cached<CachedGuildMember>>,
    pub group: Option<MemberListGroup>,
}

#[derive(Clone, Debug)]
pub struct CachedMemberListGuildMember {
    pub presence: Option<MemberListPresence>,
    pub member: Cached<CachedGuildMember>,
}
