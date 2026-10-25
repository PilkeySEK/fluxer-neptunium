use mini_moka::sync::ConcurrentCacheExt;

use crate::Cache;

#[derive(Copy, Clone, Debug)]
pub struct CacheStats {
    pub total_objects: u64,
    pub users: u64,
    pub user_profiles: u64,
    pub channels: u64,
    pub messages: u64,
    pub invites: u64,
    pub guilds: u64,
    pub roles: u64,
}

impl CacheStats {
    pub(crate) fn calculate_from_cache(cache: &Cache) -> Self {
        cache.users.sync();
        let users = cache.users.entry_count();
        cache.user_profiles.sync();
        let user_profiles = cache.user_profiles.entry_count();
        cache.channels.sync();
        let channels = cache.channels.entry_count();
        cache.messages.sync();
        let messages = cache.messages.entry_count();
        cache.invites.sync();
        let invites = cache.invites.entry_count();
        cache.guilds.sync();
        let guilds = cache.guilds.entry_count();
        cache.roles.sync();
        let roles = cache.roles.entry_count();
        let total_objects = users + user_profiles + channels + messages + invites + guilds + roles;
        Self {
            total_objects,
            users,
            user_profiles,
            channels,
            messages,
            invites,
            guilds,
            roles,
        }
    }
}
