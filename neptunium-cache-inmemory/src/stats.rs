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
        let users = cache.users.entry_count();
        let user_profiles = cache.user_profiles.entry_count();
        let channels = cache.channels.entry_count();
        let messages = cache.messages.entry_count();
        let invites = cache.invites.entry_count();
        let guilds = cache.guilds.entry_count();
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
