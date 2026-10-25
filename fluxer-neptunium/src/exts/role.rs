use async_trait::async_trait;
use neptunium_cache_inmemory::{Cached, CachedGuildRole};
use neptunium_http::endpoints::guild::UpdateGuildRoleBody;

use crate::{client::error::ClientError, events::context::Context, exts::GuildExt};

#[async_trait]
pub trait GuildRoleExt {
    /// Update this role.
    async fn update(
        &self,
        ctx: &Context,
        updates: UpdateGuildRoleBody,
    ) -> Result<Cached<CachedGuildRole>, ClientError>;
    /// Update this role with a reason for the audit log.
    async fn update_with_reason(
        &self,
        ctx: &Context,
        updates: UpdateGuildRoleBody,
        reason: impl Into<String> + Send,
    ) -> Result<Cached<CachedGuildRole>, ClientError>;
    /// Delete this role.
    async fn delete(&self, ctx: &Context) -> Result<(), ClientError>;
    /// Delete this role with a reason for the audit log.
    async fn delete_with_reason(
        &self,
        ctx: &Context,
        reason: impl Into<String> + Send,
    ) -> Result<(), ClientError>;
}

#[async_trait]
impl GuildRoleExt for CachedGuildRole {
    async fn update(
        &self,
        ctx: &Context,
        updates: UpdateGuildRoleBody,
    ) -> Result<Cached<CachedGuildRole>, ClientError> {
        self.guild_id.update_role(ctx, self.id, updates).await
    }
    async fn update_with_reason(
        &self,
        ctx: &Context,
        updates: UpdateGuildRoleBody,
        reason: impl Into<String> + Send,
    ) -> Result<Cached<CachedGuildRole>, ClientError> {
        self.guild_id
            .update_role_with_reason(ctx, self.id, updates, reason)
            .await
    }
    async fn delete(&self, ctx: &Context) -> Result<(), ClientError> {
        self.guild_id.delete_role(ctx, self.id).await
    }
    async fn delete_with_reason(
        &self,
        ctx: &Context,
        reason: impl Into<String> + Send,
    ) -> Result<(), ClientError> {
        self.guild_id
            .delete_role_with_reason(ctx, self.id, reason)
            .await
    }
}
