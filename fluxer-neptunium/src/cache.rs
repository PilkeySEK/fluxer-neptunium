use std::{fmt::Debug, sync::Arc};

use bon::Builder;
use neptunium_model::{
    channel::{Channel, message::Message},
    gateway::payload::incoming::UserPrivateResponse,
    guild::Guild,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
    },
    user::PartialUser,
};

use quick_cache::sync::Cache as QuickCache;
use tokio::sync::RwLock;

// Things to be cached in the future: relationships, guild members, user guild settings, emojis, stickers, saved messages, (scheduled messages), user settings
#[derive(Clone)]
pub(crate) struct Cache {
    pub users: Arc<QuickCache<Id<UserMarker>, Arc<PartialUser>>>,
    pub guilds: Arc<QuickCache<Id<GuildMarker>, Arc<Guild>>>,
    pub messages: Arc<QuickCache<Id<MessageMarker>, Arc<Message>>>,
    pub channels: Arc<QuickCache<Id<ChannelMarker>, Arc<Channel>>>,
    pub current_user: Arc<tokio::sync::RwLock<Option<Arc<UserPrivateResponse>>>>,
}

impl Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cache { ... }")
    }
}

pub(crate) trait CacheKey: Copy {
    type Val;
    async fn get_or_try_compute_insert<E, F: AsyncFn() -> Result<Self::Val, E>>(
        self,
        cache: &Cache,
        insert_fn: F,
    ) -> Result<Arc<Self::Val>, E>;
    fn remove(&self, cache: &Cache) -> Option<Arc<Self::Val>>;
}

pub(crate) trait CacheValue: Sized {
    fn insert(self, cache: &Cache) -> Arc<Self>;
    fn batch_insert(vec: Vec<Self>, cache: &Cache) -> Vec<Arc<Self>>;
}

#[derive(Copy, Clone, Debug, Builder)]
#[builder(const)]
pub struct CacheConfig {
    #[builder(default = 1024)]
    pub users: usize,
    #[builder(default = 1024)]
    pub guilds: usize,
    #[builder(default = 1024)]
    pub messages: usize,
    #[builder(default = 1024)]
    pub channels: usize,
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
            users: Arc::new(quick_cache::sync::Cache::new(config.users)),
            guilds: Arc::new(quick_cache::sync::Cache::new(config.guilds)),
            messages: Arc::new(quick_cache::sync::Cache::new(config.messages)),
            channels: Arc::new(quick_cache::sync::Cache::new(config.channels)),
            current_user: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_or_try_compute_insert<K: CacheKey, E, F: AsyncFn() -> Result<K::Val, E>>(
        &self,
        key: K,
        insert_fn: F,
    ) -> Result<Arc<K::Val>, E> {
        key.get_or_try_compute_insert(self, insert_fn).await
    }

    // Not marking this as #[must_use] because removing something in the API almost never has the API return the original value either.
    pub fn remove<K: CacheKey>(&self, item: K) -> Option<Arc<K::Val>> {
        item.remove(self)
    }

    #[must_use]
    pub fn insert<V: CacheValue>(&self, value: V) -> Arc<V> {
        value.insert(self)
    }

    #[must_use]
    pub fn batch_insert<V: CacheValue>(&self, vec: Vec<V>) -> Vec<Arc<V>> {
        V::batch_insert(vec, self)
    }
}

macro_rules! impl_cache_keys {
    (
        $(
            $key_type:ty => $member:ident: $value_type:ty
        );+ ;
    ) => {
        $(
            impl CacheKey for $key_type {
                type Val = $value_type;
                async fn get_or_try_compute_insert<E, F: AsyncFn() -> Result<Self::Val, E>>(
                    self,
                    cache: &Cache,
                    insert_fn: F,
                ) -> Result<Arc<Self::Val>, E> {
                    match cache.$member.get_value_or_guard_async(&self).await {
                        Ok(value) => Ok(value),
                        Err(guard) => {
                            let value = insert_fn().await?;
                            let value = Arc::new(value);
                            if let Err(value) = guard.insert(Arc::clone(&value)) {
                                // If inserting via the guard failed (relatively unlikely), insert normally
                                cache.$member.insert(self, value);
                            }
                            Ok(value)
                        }
                    }
                }
                fn remove(&self, cache: &Cache) -> Option<Arc<Self::Val>> {
                    cache.$member.remove(self).map(|(_key, value)| value)
                }
            }
        )+
    };
}

macro_rules! impl_cache_values {
    (
        $(
            $value_type:ty => $member:ident: $key_member:ident
        );+ ;
    ) => {
        $(
            impl CacheValue for $value_type {
                fn batch_insert(vec: Vec<Self>, cache: &Cache) -> Vec<Arc<Self>> {
                    let mut arc_values = Vec::with_capacity(vec.len());
                    for value in vec {
                        let value = Arc::new(value);
                        arc_values.push(Arc::clone(&value));
                        cache.$member.insert(value.$key_member, value);
                    }
                    arc_values
                }
                fn insert(self, cache: &Cache) -> Arc<Self> {
                    let arc_self = Arc::new(self);
                    cache.$member.insert(arc_self.$key_member, Arc::clone(&arc_self));
                    arc_self
                }
            }
        )+
    };
}

impl_cache_keys!(
    Id<UserMarker> => users: PartialUser;
    Id<GuildMarker> => guilds: Guild;
    Id<MessageMarker> => messages: Message;
    Id<ChannelMarker> => channels: Channel;
);

impl_cache_values!(
    PartialUser => users: id;
    Guild => guilds: id;
    Message => messages: id;
    Channel => channels: id;
);
