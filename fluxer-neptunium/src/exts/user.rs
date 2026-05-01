use std::collections::HashMap;

use crate::{
    client::error::Error,
    events::context::Context,
    exts::{GuildExt, IntoCachedChannel},
    internal::traits::user::UserTrait,
};
use async_trait::async_trait;
use neptunium_cache_inmemory::{
    CachableEndpoint, Cached, CachedGuildMember, CachedUserProfileFullResponse,
};
use neptunium_http::endpoints::users::{GetUserById, GetUserProfile, GetUserProfileParams};
#[cfg(feature = "user_api")]
use neptunium_model::user::relationship::Relationship;
use neptunium_model::{
    channel::{PermissionOverwrite, PermissionOverwriteEntity},
    guild::permissions::Permissions,
    id::{
        Id,
        marker::{GenericMarker, RoleMarker},
    },
    user::PartialUser,
};

#[async_trait]
pub trait UserExt {
    #[cfg(feature = "user_api")]
    async fn send_friend_request(&self, ctx: &Context) -> Result<Relationship, Error>;
    /// Creates or updates a private note on this user.
    /// Pass `None` for the `note` to clear the note.
    #[cfg(feature = "user_api")]
    async fn set_user_note(&self, ctx: &Context, note: Option<String>) -> Result<(), Error>;
    /// Retrieves a specific note the current user has written about this user.
    #[cfg(feature = "user_api")]
    async fn get_user_note(&self, ctx: &Context) -> Result<String, Error>;
    /// Removes a relationship with another user by ID. Removes friends, cancels
    /// friend requests (incoming or outgoing), or unblocks a blocked user
    /// depending on current relationship type.
    #[cfg(feature = "user_api")]
    async fn remove_relationship(&self, ctx: &Context) -> Result<(), Error>;
    #[cfg(feature = "user_api")]
    async fn update_friend_nickname(
        &self,
        ctx: &Context,
        nickname: Option<String>,
    ) -> Result<Relationship, Error>;
    /// May respect privacy settings.
    async fn get_profile(
        &self,
        ctx: &Context,
        params: GetUserProfileParams,
    ) -> Result<Cached<CachedUserProfileFullResponse>, Error>;
    async fn get_user(&self, ctx: &Context) -> Result<Cached<PartialUser>, Error>;
}

#[async_trait]
impl<T: UserTrait> UserExt for T {
    #[cfg(feature = "user_api")]
    async fn send_friend_request(&self, ctx: &Context) -> Result<Relationship, Error> {
        use neptunium_http::endpoints::users::SendFriendRequest;

        Ok(ctx
            .get_http_client()
            .execute(SendFriendRequest {
                user_id: self.get_user_id(),
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn set_user_note(&self, ctx: &Context, note: Option<String>) -> Result<(), Error> {
        use neptunium_http::endpoints::users::SetUserNote;

        Ok(ctx
            .get_http_client()
            .execute(SetUserNote {
                user_id: self.get_user_id(),
                note,
            })
            .await?)
    }

    /// Retrieves a specific note the current user has written about another user.
    #[cfg(feature = "user_api")]
    async fn get_user_note(&self, ctx: &Context) -> Result<String, Error> {
        use neptunium_http::endpoints::users::GetUserNote;

        let response = ctx
            .get_http_client()
            .execute(GetUserNote {
                user_id: self.get_user_id(),
            })
            .await?;
        Ok(response.note)
    }

    #[cfg(feature = "user_api")]
    async fn remove_relationship(&self, ctx: &Context) -> Result<(), Error> {
        use neptunium_http::endpoints::users::RemoveRelationship;

        Ok(ctx
            .get_http_client()
            .execute(RemoveRelationship {
                user_id: self.get_user_id(),
            })
            .await?)
    }

    /// Updates the nickname associated with a relationship.
    /// Nicknames are personal labels that override the user’s display name in the current user’s view.
    #[cfg(feature = "user_api")]
    async fn update_friend_nickname(
        &self,
        ctx: &Context,
        nickname: Option<String>,
    ) -> Result<Relationship, Error> {
        use neptunium_http::endpoints::users::UpdateRelationshipNickname;

        Ok(ctx
            .get_http_client()
            .execute(UpdateRelationshipNickname {
                nickname,
                user_id: self.get_user_id(),
            })
            .await?)
    }

    async fn get_profile(
        &self,
        ctx: &Context,
        params: GetUserProfileParams,
    ) -> Result<Cached<CachedUserProfileFullResponse>, Error> {
        Ok(GetUserProfile {
            user_id: self.get_user_id(),
            params,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn get_user(&self, ctx: &Context) -> Result<Cached<PartialUser>, Error> {
        Ok(GetUserById {
            user_id: self.get_user_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }
}

pub trait PartialUserExt {
    /// Returns either the display name (global name), or
    /// the username if a global name is not set.
    fn display_name(&self) -> String;
    /// Returns either the display name (global name) with the specified `global_name_prefix`,
    /// or the username with the specified `username_prefix` if a global name is not set.
    fn display_name_formatted(&self, global_name_prefix: &str, username_prefix: &str) -> String;
}

impl PartialUserExt for PartialUser {
    fn display_name(&self) -> String {
        if let Some(global_name) = &self.global_name {
            global_name.clone()
        } else {
            self.username.clone()
        }
    }

    fn display_name_formatted(&self, global_name_prefix: &str, username_prefix: &str) -> String {
        if let Some(global_name) = &self.global_name {
            format!("{global_name_prefix}{global_name}")
        } else {
            format!("{}{}", username_prefix, self.username)
        }
    }
}

#[async_trait]
pub trait GuildMemberExt {
    /// Calculate the member's permissions based on their roles (and the permissions of `@everyone`).
    /// Does not take into account whether the member is the guild owner or has Administrator permissions (and thus would
    /// bypass all permission checks).
    ///
    /// # Errors
    /// Because it (at least currently) cannot be guaranteed that the guild roles are already cached,
    /// the roles may be fetched from the API, which can produce network errors.
    async fn get_permissions(&self, ctx: &Context) -> Result<Permissions, Error>;
    /// Calculate the member's permissions in the given channel. The permissions are calculated with the following precedence
    /// (from lowest to highest precedence):
    ///
    /// - Guild role permissions (including the `@everyone` role)
    /// - Channel overrides of `@everyone`
    /// - Channel overrides of the member's roles (the role positions are not taken into account)
    /// - Channel overrides specifically for this member
    ///
    /// You can think of these precendences as levels of precision. The higher the precision, the higher the precedence.
    ///
    /// There are additional non-obvious rules for calculating overwrites:
    ///
    /// - An "allow" will override a "deny" on the same precedence level.
    /// - A "deny" on a higher precedence level will override an "allow" on a lower precedence level.
    ///
    /// Returns empty permissions if the provided channel is not a guild channel.
    /// This function does not take into account whether the member is the guild owner or has
    /// Administrator permissions (and thus would bypass all permission checks).
    ///
    /// # Errors
    /// Because it (at least currently) cannot be guaranteed that the guild roles are already cached,
    /// the roles may be fetched from the API, which can produce network errors.
    /// The provided channel may also need to be fetched from the API (for example, if an `Id<ChannelMarker>` was passed to
    /// this function).
    async fn get_permissions_in_channel<T: IntoCachedChannel + Send>(
        &self,
        ctx: &Context,
        channel: T,
    ) -> Result<Permissions, Error>;
    /// Whether this member is the guild owner.
    async fn is_guild_owner(&self, ctx: &Context) -> Result<bool, Error>;
    /// Calculate whether the member has the provided permissions based on their roles (and the permissions of `@everyone`).
    /// If the member is the guild owner or has administrator permissions, this will always return `true`.
    ///
    /// # Errors
    /// Because it (at least currently) cannot be guaranteed that the guild roles are already cached,
    /// the roles may be fetched from the API, which can produce network errors.
    async fn has_permissions(&self, ctx: &Context, permissions: Permissions)
    -> Result<bool, Error>;
    /// Calculate whether the member has the provided permissions based on their roles (and the permissions of `@everyone`)
    /// and the permission overwrites of the given channel.
    /// If the member is the guild owner or has administrator permissions, this will always return `true`.
    ///
    /// # Errors
    /// Because it (at least currently) cannot be guaranteed that the guild roles are already cached,
    /// the roles may be fetched from the API, which can produce network errors.
    /// The provided channel may also need to be fetched from the API (for example, if an `Id<ChannelMarker>` was passed to
    /// this function).
    async fn has_permissions_in_channel<T: IntoCachedChannel + Send>(
        &self,
        ctx: &Context,
        channel: T,
        permissions: Permissions,
    ) -> Result<bool, Error>;
    async fn add_role(&self, ctx: &Context, role_id: Id<RoleMarker>) -> Result<(), Error>;
    async fn remove_role(&self, ctx: &Context, role_id: Id<RoleMarker>) -> Result<(), Error>;
}

#[async_trait]
impl GuildMemberExt for CachedGuildMember {
    async fn get_permissions(&self, ctx: &Context) -> Result<Permissions, Error> {
        let own_role_ids = &self.roles;
        let guild_roles_permissions = self
            .guild_id
            .list_roles(ctx)
            .await?
            .into_iter()
            .map(|role| {
                let role = role.load();
                (role.id, role.permissions)
            })
            .collect::<HashMap<_, _>>();

        let everyone_permissions = guild_roles_permissions.get(&self.guild_id.cast()).copied();

        let mut permissions = own_role_ids
            .iter()
            .fold(Permissions::empty(), |acc, role_id| {
                let Some(role_permissions) = guild_roles_permissions.get(role_id) else {
                    tracing::warn!(
                        "Role with ID {} is present on user but was not found in the guild.",
                        role_id
                    );
                    return acc;
                };
                acc.union(*role_permissions)
            });
        if let Some(everyone_permissions) = everyone_permissions {
            permissions = permissions.union(everyone_permissions);
        } else {
            tracing::warn!("Role for [@]everyone not present in guild roles.");
        }
        Ok(permissions)
    }

    async fn get_permissions_in_channel<T: IntoCachedChannel + Send>(
        &self,
        ctx: &Context,
        channel: T,
    ) -> Result<Permissions, Error> {
        let mut calculated_permissions = self.get_permissions(ctx).await?;
        let channel_permission_overwrites = {
            let channel = channel.into_cached_channel(ctx).await?;
            channel
                .load()
                .permission_overwrites
                .as_deref()
                .map(|overwrites| {
                    overwrites
                        .iter()
                        .map(|overwrite| (overwrite.id, *overwrite))
                        .collect::<HashMap<Id<GenericMarker>, PermissionOverwrite>>()
                })
        };
        let Some(mut channel_permission_overwrites) = channel_permission_overwrites else {
            return Ok(Permissions::empty());
        };
        let everyone_role_id = self.guild_id.cast::<GenericMarker>();
        if let Some(everyone_permission_overwrite) =
            channel_permission_overwrites.remove(&everyone_role_id)
        {
            calculated_permissions =
                calculated_permissions.difference(everyone_permission_overwrite.deny);
            calculated_permissions =
                calculated_permissions.union(everyone_permission_overwrite.allow);
        }

        let (role_overwrites, member_overwrites): (Vec<_>, Vec<_>) = channel_permission_overwrites
            .into_values()
            .partition(|overwrite| overwrite.r#type == PermissionOverwriteEntity::Role);

        calculated_permissions = role_overwrites
            .into_iter()
            .fold(calculated_permissions, |acc, overwrite| {
                acc.difference(overwrite.deny).union(overwrite.allow)
            });

        if let Some(own_overwrite) = member_overwrites
            .into_iter()
            .find(|overwrite| overwrite.id == self.id.cast())
        {
            calculated_permissions = calculated_permissions
                .difference(own_overwrite.deny)
                .union(own_overwrite.allow);
        }

        Ok(calculated_permissions)
    }

    async fn is_guild_owner(&self, ctx: &Context) -> Result<bool, Error> {
        let guild = self.guild_id.fetch(ctx).await?.load();
        Ok(self.id == guild.owner_id)
    }

    async fn has_permissions(
        &self,
        ctx: &Context,
        permissions: Permissions,
    ) -> Result<bool, Error> {
        if self.is_guild_owner(ctx).await? {
            return Ok(true);
        }
        let member_permissions = self.get_permissions(ctx).await?;
        if member_permissions.contains(Permissions::ADMINISTRATOR) {
            return Ok(true);
        }
        Ok(member_permissions.contains(permissions))
    }

    async fn has_permissions_in_channel<T: IntoCachedChannel + Send>(
        &self,
        ctx: &Context,
        channel: T,
        permissions: Permissions,
    ) -> Result<bool, Error> {
        if self.is_guild_owner(ctx).await? {
            return Ok(true);
        }
        let member_permissions = self.get_permissions_in_channel(ctx, channel).await?;
        if member_permissions.contains(Permissions::ADMINISTRATOR) {
            return Ok(true);
        }
        Ok(member_permissions.contains(permissions))
    }

    async fn add_role(&self, ctx: &Context, role_id: Id<RoleMarker>) -> Result<(), Error> {
        self.guild_id
            .add_role_to_member(ctx, self.id, role_id)
            .await
    }

    async fn remove_role(&self, ctx: &Context, role_id: Id<RoleMarker>) -> Result<(), Error> {
        self.guild_id
            .remove_role_from_member(ctx, self.id, role_id)
            .await
    }
}
