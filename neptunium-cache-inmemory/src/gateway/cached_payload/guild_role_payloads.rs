use neptunium_model::{
    gateway::payload::incoming::{GuildRoleCreate, GuildRoleUpdateBulk},
    guild::permissions::GuildRole,
};

use crate::{CacheValue, Cached, gateway::cached_payload::CachedPayload};

impl CachedPayload for Cached<GuildRole> {
    type NonCached = GuildRoleCreate;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        non_cached.role.insert_and_return(cache)
    }
}

pub struct CachedGuildRoleUpdateBulk {
    pub roles: Vec<Cached<GuildRole>>,
}

impl CachedPayload for CachedGuildRoleUpdateBulk {
    type NonCached = GuildRoleUpdateBulk;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        Self {
            roles: non_cached
                .roles
                .into_iter()
                .map(|role| role.insert_and_return(cache))
                .collect(),
        }
    }
}
