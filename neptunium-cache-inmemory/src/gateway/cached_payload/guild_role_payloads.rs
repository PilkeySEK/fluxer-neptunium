use neptunium_model::gateway::payload::incoming::{GuildRoleCreate, GuildRoleUpdateBulk};

use crate::{CacheValue, Cached, CachedGuildRole, gateway::cached_payload::CachedPayload};

impl CachedPayload for Cached<CachedGuildRole> {
    type NonCached = GuildRoleCreate;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        CachedGuildRole::from_guild_role(non_cached.role, non_cached.guild_id)
            .insert_and_return(cache)
    }
}

pub struct CachedGuildRoleUpdateBulk {
    pub roles: Vec<Cached<CachedGuildRole>>,
}

impl CachedPayload for CachedGuildRoleUpdateBulk {
    type NonCached = GuildRoleUpdateBulk;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        Self {
            roles: non_cached
                .roles
                .into_iter()
                .map(|role| {
                    CachedGuildRole::from_guild_role(role, non_cached.guild_id)
                        .insert_and_return(cache)
                })
                .collect(),
        }
    }
}
