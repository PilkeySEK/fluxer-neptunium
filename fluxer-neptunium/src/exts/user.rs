use crate::{client::error::Error, events::context::Context, internal::traits::user::UserTrait};
use async_trait::async_trait;
use neptunium_cache_inmemory::{CachableEndpoint, Cached, CachedUserProfileFullResponse};
use neptunium_http::endpoints::users::{GetUserById, GetUserProfile, GetUserProfileParams};
use neptunium_model::user::PartialUser;
#[cfg(feature = "user_api")]
use neptunium_model::user::relationship::Relationship;

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
