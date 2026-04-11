use std::{fmt::Debug, sync::Arc};

use arc_swap::ArcSwap;
use atomic_once_cell::AtomicOnceCell;
use bon::Builder;
use mini_moka::sync::Cache as MokaCache;
use neptunium_http::endpoints::users::UserProfileFullResponse;
use neptunium_model::{
    gateway::payload::incoming::UserPrivateResponse,
    guild::{Guild, member::GuildMemberProfile, permissions::GuildRole},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, RoleMarker, UserMarker},
    },
    invites::InviteWithMetadata,
    user::{PartialUser, UserProfileData, settings::UserSettings},
};

pub mod gateway;
#[cfg(feature = "statistics")]
pub mod stats;
mod structs;
mod traits;
pub use structs::*;
pub use traits::*;

#[cfg(feature = "statistics")]
use crate::stats::CacheStats;

// pub type Cached<T> = Arc<tokio::sync::RwLock<T>>;

pub use arc_swap::Guard;

pub struct Cached<T> {
    inner: Arc<ArcSwap<T>>,
}

impl<T> Cached<T> {
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(ArcSwap::new(Arc::new(value))),
        }
    }

    /// Get the latest value. This will be the latest value until a new value is stored.
    #[must_use]
    pub fn load(&self) -> Guard<Arc<T>> {
        self.inner.load()
    }

    /// Store a new value. All new `load()` operations will return this new value
    /// (until it is overwritten again), but existing `Guard`s will still hold the old value.
    pub fn store(&self, new_value: Arc<T>) {
        self.inner.store(new_value);
    }

    /// Modify the value "in-place". This does not actually modify the value in-place, but rather create
    /// a clone of the value which it passes to the specified `modify_fn`, then store it.
    pub fn modify(&self, modify_fn: impl FnOnce(&mut T))
    where
        T: Clone,
    {
        let mut value = self.clone_inner();
        modify_fn(&mut value);
        self.store(Arc::new(value));
    }

    /// Only stores the modified value if `modify_fn` returns `Ok(())`.
    /// # Errors
    /// Returns the result of the closure.
    pub fn try_modify<E>(&self, modify_fn: impl FnOnce(&mut T) -> Result<(), E>) -> Result<(), E>
    where
        T: Clone,
    {
        let mut value = self.clone_inner();
        let result = modify_fn(&mut value);
        if result.is_ok() {
            self.store(Arc::new(value));
        }
        result
    }

    /// Stores the value and then returns a clone of `Self`.
    #[must_use]
    pub fn store_and_return(&self, new_value: T) -> Self {
        self.inner.store(Arc::new(new_value));
        self.clone()
    }

    /// Clones the stored value and returns it.
    #[must_use]
    pub fn clone_inner(&self) -> T
    where
        T: Clone,
    {
        (*(*self.load())).clone()
    }
}

impl<T> Debug for Cached<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cached { ... }")
    }
}

impl<T> Clone for Cached<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

// TODO: More things to cache: guild channels (guild id->channels), relationships,
// guild invites (guild id->invites), guild members
#[expect(clippy::type_complexity)]
pub struct Cache {
    pub users: MokaCache<Id<UserMarker>, Cached<PartialUser>>,
    pub guild_members: MokaCache<Id<GuildMarker>, Vec<Cached<CachedGuildMember>>>,
    pub user_profile_data: MokaCache<Id<UserMarker>, Cached<UserProfileData>>,
    pub guild_member_profiles:
        MokaCache<(Id<UserMarker>, Id<GuildMarker>), Cached<GuildMemberProfile>>,
    pub user_profiles:
        MokaCache<(Id<UserMarker>, Option<Id<GuildMarker>>), Cached<UserProfileFullResponse>>,
    pub channels: MokaCache<Id<ChannelMarker>, Cached<CachedChannel>>,
    pub messages: MokaCache<Id<MessageMarker>, Cached<CachedMessage>>,
    pub current_user: AtomicOnceCell<Cached<UserPrivateResponse>>,
    pub current_user_settings: AtomicOnceCell<Cached<UserSettings>>,
    pub invites: MokaCache<String, Cached<InviteWithMetadata>>,
    pub guilds: MokaCache<Id<GuildMarker>, Cached<Guild>>,
    // TODO: Attach guild id
    pub roles: MokaCache<Id<RoleMarker>, Cached<GuildRole>>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct CacheConfig {
    #[builder(default = u64::MAX)]
    pub users: u64,
    #[builder(default = u64::MAX)]
    pub user_profiles: u64,
    #[builder(default = u64::MAX)]
    pub channels: u64,
    #[builder(default = 4096)]
    pub messages: u64,
    #[builder(default = 4096)]
    pub invites: u64,
    #[builder(default = u64::MAX)]
    pub guilds: u64,
    #[builder(default = u64::MAX)]
    pub roles: u64,
    #[builder(default = u64::MAX)]
    pub guild_members: u64,
    #[builder(default = 8192)]
    pub user_profile_data: u64,
    #[builder(default = 8192)]
    pub guild_member_profiles: u64,
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
            current_user: AtomicOnceCell::new(),
            current_user_settings: AtomicOnceCell::new(),
            invites: MokaCache::new(config.invites),
            guilds: MokaCache::new(config.guilds),
            roles: MokaCache::new(config.roles),
            guild_member_profiles: MokaCache::new(config.guild_member_profiles),
            guild_members: MokaCache::new(config.guild_members),
            user_profile_data: MokaCache::new(config.user_profile_data),
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
