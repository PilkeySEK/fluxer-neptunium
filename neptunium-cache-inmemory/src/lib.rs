use std::{fmt::Debug, sync::Arc};

use bon::Builder;
use mini_moka::sync::Cache as MokaCache;
use neptunium_http::endpoints::users::UserProfileFullResponse;
use neptunium_model::{
    gateway::payload::incoming::UserPrivateResponse,
    guild::{Guild, permissions::GuildRole},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, RoleMarker, UserMarker},
    },
    invites::InviteWithMetadata,
    user::{PartialUser, settings::UserSettings},
};
use tokio::sync::OnceCell;

pub mod gateway;
#[cfg(feature = "statistics")]
pub mod stats;
mod structs;
mod traits;
pub use structs::*;
pub use traits::*;

#[cfg(feature = "statistics")]
use crate::stats::CacheStats;

pub type Cached<T> = Arc<tokio::sync::RwLock<T>>;

// TODO: More things to cache: guild channels (guild id->channels), relationships,
// guild invites (guild id->invites), guild members
#[expect(clippy::type_complexity)]
pub struct Cache {
    pub users: MokaCache<Id<UserMarker>, Cached<PartialUser>>,
    pub user_profiles:
        MokaCache<(Id<UserMarker>, Option<Id<GuildMarker>>), Cached<UserProfileFullResponse>>,
    pub channels: MokaCache<Id<ChannelMarker>, Cached<CachedChannel>>,
    pub messages: MokaCache<Id<MessageMarker>, Cached<CachedMessage>>,
    pub current_user: OnceCell<Cached<UserPrivateResponse>>,
    pub current_user_settings: OnceCell<Cached<UserSettings>>,
    pub invites: MokaCache<String, Cached<InviteWithMetadata>>,
    pub guilds: MokaCache<Id<GuildMarker>, Cached<Guild>>,
    // TODO: Attach guild id
    pub roles: MokaCache<Id<RoleMarker>, Cached<GuildRole>>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct CacheConfig {
    #[builder(default = 1024)]
    pub users: u64,
    #[builder(default = 256)]
    pub user_profiles: u64,
    #[builder(default = 1024)]
    pub channels: u64,
    #[builder(default = 1024)]
    pub messages: u64,
    #[builder(default = 256)]
    pub invites: u64,
    #[builder(default = 1024)]
    pub guilds: u64,
    #[builder(default = 1024)]
    pub roles: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Cache {
    #[must_use]
    pub fn new(config: CacheConfig) -> Self {
        Self {
            users: MokaCache::new(config.users),
            user_profiles: MokaCache::new(config.user_profiles),
            channels: MokaCache::new(config.channels),
            messages: MokaCache::new(config.messages),
            current_user: OnceCell::new(),
            current_user_settings: OnceCell::new(),
            invites: MokaCache::new(config.invites),
            guilds: MokaCache::new(config.guilds),
            roles: MokaCache::new(config.roles),
        }
    }

    /// Calculate approximate statistics about the cache.
    #[cfg(feature = "statistics")]
    #[must_use]
    pub fn stats(&self) -> CacheStats {
        CacheStats::calculate_from_cache(self)
    }
}

impl Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cache { ... }")
    }
}
