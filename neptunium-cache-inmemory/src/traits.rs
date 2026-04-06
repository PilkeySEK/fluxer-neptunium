use std::sync::Arc;

use crate::{Cache, Cached, CachedChannel, CachedMessage};
use async_trait::async_trait;
use neptunium_http::{
    client::HttpClient,
    endpoints::{Endpoint, ExecuteEndpointRequestError},
};
use neptunium_model::{
    gateway::payload::incoming::UserPrivateResponse,
    guild::{Guild, permissions::GuildRole},
    invites::{GroupDmInvite, GuildInvite, InviteWithMetadata, PackInvite},
    user::PartialUser,
};
use tokio::sync::RwLock;

pub mod cachable_endpoints;

pub(crate) trait CacheValue {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self>;
}

#[async_trait]
pub trait CachableEndpoint: Endpoint {
    type Response;
    /// Either get the result from the cache or execute the request.
    /// # Errors
    /// Returns an error if the HTTP request fails or parsing the response fails.
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>>;
}

impl CacheValue for CachedChannel {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let channel_id = self.id;
        if let Some(existing_channel) = cache.channels.get(&channel_id) {
            {
                let mut guard = existing_channel.write().await;
                *guard = self;
            }
            return existing_channel;
        }
        let value = Arc::new(RwLock::new(self));
        cache.channels.insert(channel_id, Arc::clone(&value));
        value
    }
}

impl CacheValue for CachedMessage {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let message_id = self.id;
        if let Some(existing_message) = cache.messages.get(&message_id) {
            {
                let mut guard = existing_message.write().await;
                *guard = self;
            }
            return existing_message;
        }
        let value = Arc::new(RwLock::new(self));
        cache.messages.insert(message_id, Arc::clone(&value));
        value
    }
}

impl CacheValue for UserPrivateResponse {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        if let Some(existing_user) = cache.current_user.get() {
            let existing_user = Arc::clone(existing_user);
            {
                let mut guard = existing_user.write().await;
                *guard = self;
            }
            existing_user
        } else {
            Arc::clone(
                cache
                    .current_user
                    .get_or_init(async || Arc::new(RwLock::new(self)))
                    .await,
            )
        }
    }
}

impl CacheValue for InviteWithMetadata {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let code = match &self {
            Self::EmojiPack(PackInvite { code, .. }, _)
            | Self::GroupDm(GroupDmInvite { code, .. }, _)
            | Self::Guild(GuildInvite { code, .. }, _)
            | Self::StickerPack(PackInvite { code, .. }, _) => code.clone(),
        };
        if let Some(cached_invite) = cache.invites.get(&code) {
            {
                let mut guard = cached_invite.write().await;
                *guard = self;
            }
            return cached_invite;
        }
        let cached_self = Arc::new(RwLock::new(self));
        cache.invites.insert(code, Arc::clone(&cached_self));
        cached_self
    }
}

impl CacheValue for PartialUser {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let user_id = self.id;
        if let Some(existing_user) = cache.users.get(&user_id) {
            {
                let mut guard = existing_user.write().await;
                *guard = self;
            }
            return existing_user;
        }
        let value = Arc::new(RwLock::new(self));
        cache.users.insert(user_id, Arc::clone(&value));
        value
    }
}

impl CacheValue for neptunium_model::user::settings::UserSettings {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        if let Some(existing_settings) = cache.current_user_settings.get() {
            let existing_settings = Arc::clone(existing_settings);
            {
                let mut guard = existing_settings.write().await;
                *guard = self;
            }
            existing_settings
        } else {
            Arc::clone(
                cache
                    .current_user_settings
                    .get_or_init(async || Arc::new(RwLock::new(self)))
                    .await,
            )
        }
    }
}

impl CacheValue for Guild {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let guild_id = self.id;
        if let Some(existing_guild) = cache.guilds.get(&guild_id) {
            {
                let mut guard = existing_guild.write().await;
                *guard = self;
            }
            return existing_guild;
        }
        let value = Arc::new(RwLock::new(self));
        cache.guilds.insert(guild_id, Arc::clone(&value));
        value
    }
}

impl CacheValue for GuildRole {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let role_id = self.id;
        if let Some(existing_role) = cache.roles.get(&role_id) {
            {
                let mut guard = existing_role.write().await;
                *guard = self;
            }
            return existing_role;
        }
        let value = Arc::new(RwLock::new(self));
        cache.roles.insert(role_id, Arc::clone(&value));
        value
    }
}
