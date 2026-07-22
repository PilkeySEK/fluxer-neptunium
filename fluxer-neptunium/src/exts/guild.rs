use async_trait::async_trait;
use neptunium_cache_inmemory::{
    CachableEndpoint, Cached, CachedChannel, CachedGuildMember, CachedGuildRole,
};
#[cfg(feature = "user_api")]
use neptunium_http::endpoints::users::{UpdateUserGuildSettings, UpdateUserGuildSettingsBody};
use neptunium_http::endpoints::{
    guild::{
        AddRoleToGuildMember, BanGuildMember, BanGuildMemberBody, BulkCreateGuildStickers,
        BulkCreateGuildStickersResponse, CreateGuildChannel, CreateGuildRole, CreateGuildRoleBody,
        CreateGuildSticker, CreateGuildStickerBody, DeleteGuildRole, DeleteGuildSticker,
        GetCurrentUserGuildMember, GetGuildInformation, GetGuildMember, GetGuildVanityUrl,
        GetGuildVanityUrlResponse, GuildChannelCreateRequest, KickGuildMember, LeaveGuild,
        ListGuildAuditLogs, ListGuildAuditLogsParams, ListGuildBans, ListGuildChannels,
        ListGuildMembers, ListGuildRoles, ListGuildStickers, RemoveRoleFromGuildMember,
        ResetGuildRoleHoistPositions, SearchGuildMembersMemberResponse, ToggleDetachedBanner,
        ToggleGuildTextChannelFlexibleNames, UnbanGuildMember, UpdateGuildChannelPositions,
        UpdateGuildChannelPositionsEntry, UpdateGuildMember, UpdateGuildMemberBody,
        UpdateGuildRole, UpdateGuildRoleBody, UpdateGuildRoleHoistPositions,
        UpdateGuildRoleHoistPositionsEntry, UpdateGuildRolePositions,
        UpdateGuildRolePositionsEntry, UpdateGuildSettings, UpdateGuildSettingsBody,
        UpdateGuildSticker, UpdateGuildStickerBody, UpdateGuildVanityUrl,
        UpdateGuildVanityUrlResponse,
    },
    invites::ListGuildInvites,
    webhooks::ListGuildWebhooks,
};
#[cfg(feature = "user_api")]
use neptunium_model::user::{auth::SudoVerification, settings::UserGuildSettings};
use neptunium_model::{
    guild::{
        Guild, audit_log::GuildAuditLogs, bans::GuildBanListEntry, properties::GuildSticker,
        webhook::Webhook,
    },
    id::{
        Id,
        marker::{RoleMarker, StickerMarker, UserMarker},
    },
    invites::InviteWithMetadata,
    time::timestamp::{Timestamp, representations::Iso8601},
};

use crate::{client::error::Error, events::context::Context, internal::traits::guild::GuildTrait};

#[async_trait]
pub trait GuildExt {
    async fn list_invites(&self, ctx: &Context) -> Result<Vec<Cached<InviteWithMetadata>>, Error>;
    async fn list_webhooks(&self, ctx: &Context) -> Result<Vec<Webhook>, Error>;
    async fn fetch(&self, ctx: &Context) -> Result<Cached<Guild>, Error>;
    async fn list_audit_logs(
        &self,
        ctx: &Context,
        params: ListGuildAuditLogsParams,
    ) -> Result<GuildAuditLogs, Error>;
    async fn list_bans(&self, ctx: &Context) -> Result<Vec<GuildBanListEntry>, Error>;
    async fn ban_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        body: BanGuildMemberBody,
    ) -> Result<(), Error>;
    async fn unban_member(&self, ctx: &Context, user_id: Id<UserMarker>) -> Result<(), Error>;
    async fn unban_member_with_reason(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error>;
    async fn list_channels(&self, ctx: &Context) -> Result<Vec<Cached<CachedChannel>>, Error>;
    async fn create_channel(
        &self,
        ctx: &Context,
        channel: GuildChannelCreateRequest,
    ) -> Result<Cached<CachedChannel>, Error>;
    async fn create_channel_with_reason(
        &self,
        ctx: &Context,
        channel: GuildChannelCreateRequest,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedChannel>, Error>;
    // TODO: Add helper functions for things, such as making a reordering using Vec<Id<ChannelMarker>>
    async fn update_channel_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildChannelPositionsEntry>,
    ) -> Result<(), Error>;
    async fn update_channel_positions_with_reason(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildChannelPositionsEntry>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error>;
    #[cfg(feature = "user_api")]
    async fn delete(
        &self,
        ctx: &Context,
        auth: neptunium_model::user::auth::SudoVerification,
    ) -> Result<(), Error>;
    async fn toggle_detached_banner(
        &self,
        ctx: &Context,
        detached: bool,
    ) -> Result<Cached<Guild>, Error>;
    async fn toggle_detached_banner_with_reason(
        &self,
        ctx: &Context,
        detached: bool,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<Guild>, Error>;
    /// List the guild members. `limit` defaults to 1 and should not be greater than 1000.
    async fn list_members(
        &self,
        ctx: &Context,
        limit: Option<u16>,
        after: Option<Id<UserMarker>>,
    ) -> Result<Vec<Cached<CachedGuildMember>>, Error>;
    async fn search_members(
        &self,
        ctx: &Context,
        body: neptunium_http::endpoints::guild::SearchGuildMembersBody,
    ) -> Result<neptunium_http::endpoints::guild::SearchGuildMembersResponse, Error>;
    /// Searches guild members but re-runs the search as many times as needed to accumulate all members matching the filters.
    /// In other words, this method manages the paging for you.
    async fn search_members_all(
        &self,
        ctx: &Context,
        body: neptunium_http::endpoints::guild::SearchGuildMembersBody,
    ) -> Result<Vec<SearchGuildMembersMemberResponse>, Error>;
    /// Get the authenticated bot/user as the guild member.
    async fn get_current_member(&self, ctx: &Context) -> Result<Cached<CachedGuildMember>, Error>;
    async fn update_current_member(
        &self,
        ctx: &Context,
        updates: neptunium_http::endpoints::guild::UpdateCurrentUserGuildMemberBody,
    ) -> Result<Cached<CachedGuildMember>, Error>;
    async fn get_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
    ) -> Result<Cached<CachedGuildMember>, Error>;
    async fn kick_member(&self, ctx: &Context, member_id: Id<UserMarker>) -> Result<(), Error>;
    async fn update_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        body: UpdateGuildMemberBody,
    ) -> Result<Cached<CachedGuildMember>, Error>;
    async fn update_member_with_reason(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        body: UpdateGuildMemberBody,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedGuildMember>, Error>;
    async fn timeout_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        until: impl Into<Timestamp<Iso8601>> + Send + Sync,
    ) -> Result<Cached<CachedGuildMember>, Error>;
    async fn timeout_member_with_reason(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        until: impl Into<Timestamp<Iso8601>> + Send + Sync,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedGuildMember>, Error>;
    async fn untimeout_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
    ) -> Result<Cached<CachedGuildMember>, Error>;
    async fn untimeout_member_with_reason(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedGuildMember>, Error>;
    async fn add_role_to_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<(), Error>;
    async fn add_role_to_member_with_reason(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error>;
    async fn remove_role_from_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<(), Error>;
    async fn remove_role_from_member_with_reason(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error>;
    async fn list_roles(&self, ctx: &Context) -> Result<Vec<Cached<CachedGuildRole>>, Error>;
    async fn create_role(
        &self,
        ctx: &Context,
        body: CreateGuildRoleBody,
    ) -> Result<Cached<CachedGuildRole>, Error>;
    async fn create_role_with_reason(
        &self,
        ctx: &Context,
        body: CreateGuildRoleBody,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedGuildRole>, Error>;
    async fn update_role_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRolePositionsEntry>,
    ) -> Result<(), Error>;
    async fn update_role_positions_with_reason(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRolePositionsEntry>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error>;
    async fn reset_role_hoist_positions(&self, ctx: &Context) -> Result<(), Error>;
    async fn reset_role_hoist_positions_with_reason(
        &self,
        ctx: &Context,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error>;
    async fn update_role_hoist_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRoleHoistPositionsEntry>,
    ) -> Result<(), Error>;
    async fn update_role_hoist_positions_with_reason(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRoleHoistPositionsEntry>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error>;
    async fn delete_role(&self, ctx: &Context, role_id: Id<RoleMarker>) -> Result<(), Error>;
    async fn delete_role_with_reason(
        &self,
        ctx: &Context,
        role_id: Id<RoleMarker>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error>;
    async fn update_role(
        &self,
        ctx: &Context,
        role_id: Id<RoleMarker>,
        updates: UpdateGuildRoleBody,
    ) -> Result<Cached<CachedGuildRole>, Error>;
    async fn update_role_with_reason(
        &self,
        ctx: &Context,
        role_id: Id<RoleMarker>,
        updates: UpdateGuildRoleBody,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedGuildRole>, Error>;
    async fn list_stickers(&self, ctx: &Context) -> Result<Vec<GuildSticker>, Error>;
    async fn create_sticker(
        &self,
        ctx: &Context,
        sticker: CreateGuildStickerBody,
    ) -> Result<GuildSticker, Error>;
    async fn bulk_create_stickers(
        &self,
        ctx: &Context,
        stickers: Vec<CreateGuildStickerBody>,
    ) -> Result<BulkCreateGuildStickersResponse, Error>;
    async fn delete_sticker(
        &self,
        ctx: &Context,
        sticker_id: Id<StickerMarker>,
    ) -> Result<(), Error>;
    async fn update_sticker(
        &self,
        ctx: &Context,
        sticker_id: Id<StickerMarker>,
        updates: UpdateGuildStickerBody,
    ) -> Result<GuildSticker, Error>;
    async fn toggle_channel_flexible_names(
        &self,
        ctx: &Context,
        enabled: bool,
    ) -> Result<Cached<Guild>, Error>;
    async fn toggle_channel_flexible_names_with_reason(
        &self,
        ctx: &Context,
        enabled: bool,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<Guild>, Error>;
    #[cfg(feature = "user_api")]
    async fn transfer_ownership(
        &self,
        ctx: &Context,
        new_owner_id: Id<UserMarker>,
        auth: SudoVerification,
    ) -> Result<Cached<Guild>, Error>;
    async fn get_vanity_url(&self, ctx: &Context) -> Result<GetGuildVanityUrlResponse, Error>;
    async fn update_vanity_url(
        &self,
        ctx: &Context,
        code: Option<String>,
    ) -> Result<UpdateGuildVanityUrlResponse, Error>;
    async fn update_vanity_url_with_reason(
        &self,
        ctx: &Context,
        code: Option<String>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<UpdateGuildVanityUrlResponse, Error>;
    /// Leave this guild.
    async fn leave(&self, ctx: &Context) -> Result<(), Error>;
    /// Update the guild-specific settings of the current user for this guild.
    #[cfg(feature = "user_api")]
    async fn update_user_settings(
        &self,
        ctx: &Context,
        body: UpdateUserGuildSettingsBody,
    ) -> Result<UserGuildSettings, Error>;
    async fn update_settings(
        &self,
        ctx: &Context,
        body: UpdateGuildSettingsBody,
    ) -> Result<Cached<Guild>, Error>;
    async fn update_settings_with_reason(
        &self,
        ctx: &Context,
        body: UpdateGuildSettingsBody,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<Guild>, Error>;
}

#[async_trait]
impl<T: GuildTrait> GuildExt for T {
    async fn list_invites(&self, ctx: &Context) -> Result<Vec<Cached<InviteWithMetadata>>, Error> {
        Ok(ListGuildInvites {
            guild_id: self.get_guild_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn list_webhooks(&self, ctx: &Context) -> Result<Vec<Webhook>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListGuildWebhooks {
                guild_id: self.get_guild_id(),
            })
            .await?)
    }

    async fn fetch(&self, ctx: &Context) -> Result<Cached<Guild>, Error> {
        Ok(GetGuildInformation {
            guild_id: self.get_guild_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn list_audit_logs(
        &self,
        ctx: &Context,
        params: ListGuildAuditLogsParams,
    ) -> Result<GuildAuditLogs, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListGuildAuditLogs {
                guild_id: self.get_guild_id(),
                params,
            })
            .await?)
    }

    async fn list_bans(&self, ctx: &Context) -> Result<Vec<GuildBanListEntry>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListGuildBans {
                guild_id: self.get_guild_id(),
            })
            .await?)
    }

    async fn ban_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        body: BanGuildMemberBody,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(BanGuildMember {
                guild_id: self.get_guild_id(),
                user_id: member_id,
                body,
            })
            .await?)
    }

    async fn unban_member(&self, ctx: &Context, user_id: Id<UserMarker>) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(UnbanGuildMember {
                guild_id: self.get_guild_id(),
                user_id,
                audit_log_reason: None,
            })
            .await?)
    }

    async fn unban_member_with_reason(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(UnbanGuildMember {
                guild_id: self.get_guild_id(),
                user_id,
                audit_log_reason: Some(reason.into()),
            })
            .await?)
    }

    async fn list_channels(&self, ctx: &Context) -> Result<Vec<Cached<CachedChannel>>, Error> {
        Ok(ListGuildChannels {
            guild_id: self.get_guild_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn create_channel(
        &self,
        ctx: &Context,
        channel: GuildChannelCreateRequest,
    ) -> Result<Cached<CachedChannel>, Error> {
        Ok(CreateGuildChannel {
            guild_id: self.get_guild_id(),
            body: channel,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn create_channel_with_reason(
        &self,
        ctx: &Context,
        channel: GuildChannelCreateRequest,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedChannel>, Error> {
        Ok(CreateGuildChannel {
            guild_id: self.get_guild_id(),
            body: channel,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_channel_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildChannelPositionsEntry>,
    ) -> Result<(), Error> {
        // TODO: Caching for this
        Ok(ctx
            .get_http_client()
            .execute(UpdateGuildChannelPositions {
                guild_id: self.get_guild_id(),
                body: positions,
                audit_log_reason: None,
            })
            .await?)
    }

    async fn update_channel_positions_with_reason(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildChannelPositionsEntry>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error> {
        // TODO: Caching for this
        Ok(ctx
            .get_http_client()
            .execute(UpdateGuildChannelPositions {
                guild_id: self.get_guild_id(),
                body: positions,
                audit_log_reason: Some(reason.into()),
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn delete(
        &self,
        ctx: &Context,
        auth: neptunium_model::user::auth::SudoVerification,
    ) -> Result<(), Error> {
        use neptunium_http::endpoints::guild::DeleteGuild;

        Ok(DeleteGuild {
            guild_id: self.get_guild_id(),
            auth,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn toggle_detached_banner(
        &self,
        ctx: &Context,
        detached: bool,
    ) -> Result<Cached<Guild>, Error> {
        Ok(ToggleDetachedBanner {
            guild_id: self.get_guild_id(),
            enabled: detached,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn toggle_detached_banner_with_reason(
        &self,
        ctx: &Context,
        detached: bool,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<Guild>, Error> {
        Ok(ToggleDetachedBanner {
            guild_id: self.get_guild_id(),
            enabled: detached,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn list_members(
        &self,
        ctx: &Context,
        limit: Option<u16>,
        after: Option<Id<UserMarker>>,
    ) -> Result<Vec<Cached<CachedGuildMember>>, Error> {
        Ok(ListGuildMembers {
            guild_id: self.get_guild_id(),
            limit,
            after,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn search_members(
        &self,
        ctx: &Context,
        body: neptunium_http::endpoints::guild::SearchGuildMembersBody,
    ) -> Result<neptunium_http::endpoints::guild::SearchGuildMembersResponse, Error> {
        Ok(ctx
            .get_http_client()
            .execute(neptunium_http::endpoints::guild::SearchGuildMembers {
                guild_id: self.get_guild_id(),
                body,
            })
            .await?)
    }

    async fn search_members_all(
        &self,
        ctx: &Context,
        mut body: neptunium_http::endpoints::guild::SearchGuildMembersBody,
    ) -> Result<Vec<SearchGuildMembersMemberResponse>, Error> {
        body.limit = Some(100);
        let mut members = Vec::new();

        loop {
            let mut search_result = self.search_members(ctx, body.clone()).await?;
            if let Some(offset) = &mut body.offset {
                *offset += search_result.page_result_count;
            } else {
                body.offset = Some(search_result.page_result_count);
            }
            members.append(&mut search_result.members);
            #[expect(clippy::cast_possible_truncation)]
            if search_result.total_result_count as usize <= members.len() {
                break;
            }
        }

        Ok(members)
    }

    async fn get_current_member(&self, ctx: &Context) -> Result<Cached<CachedGuildMember>, Error> {
        Ok(GetCurrentUserGuildMember {
            guild_id: self.get_guild_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_current_member(
        &self,
        ctx: &Context,
        updates: neptunium_http::endpoints::guild::UpdateCurrentUserGuildMemberBody,
    ) -> Result<Cached<CachedGuildMember>, Error> {
        use neptunium_http::endpoints::guild::UpdateCurrentUserGuildMember;

        Ok(UpdateCurrentUserGuildMember {
            guild_id: self.get_guild_id(),
            body: updates,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn get_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
    ) -> Result<Cached<CachedGuildMember>, Error> {
        Ok(GetGuildMember {
            guild_id: self.get_guild_id(),
            user_id: member_id,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn kick_member(&self, ctx: &Context, member_id: Id<UserMarker>) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(KickGuildMember {
                guild_id: self.get_guild_id(),
                user_id: member_id,
            })
            .await?)
    }

    async fn update_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        body: UpdateGuildMemberBody,
    ) -> Result<Cached<CachedGuildMember>, Error> {
        Ok(UpdateGuildMember {
            guild_id: self.get_guild_id(),
            user_id: member_id,
            body,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_member_with_reason(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        body: UpdateGuildMemberBody,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedGuildMember>, Error> {
        Ok(UpdateGuildMember {
            guild_id: self.get_guild_id(),
            user_id: member_id,
            body,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn add_role_to_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(AddRoleToGuildMember {
                guild_id: self.get_guild_id(),
                user_id: member_id,
                role_id,
                audit_log_reason: None,
            })
            .await?)
    }

    async fn add_role_to_member_with_reason(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(AddRoleToGuildMember {
                guild_id: self.get_guild_id(),
                user_id: member_id,
                role_id,
                audit_log_reason: Some(reason.into()),
            })
            .await?)
    }

    async fn remove_role_from_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(RemoveRoleFromGuildMember {
                guild_id: self.get_guild_id(),
                user_id: member_id,
                role_id,
                audit_log_reason: None,
            })
            .await?)
    }

    async fn remove_role_from_member_with_reason(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(RemoveRoleFromGuildMember {
                guild_id: self.get_guild_id(),
                user_id: member_id,
                role_id,
                audit_log_reason: Some(reason.into()),
            })
            .await?)
    }

    async fn list_roles(&self, ctx: &Context) -> Result<Vec<Cached<CachedGuildRole>>, Error> {
        Ok(ListGuildRoles {
            guild_id: self.get_guild_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn create_role(
        &self,
        ctx: &Context,
        body: CreateGuildRoleBody,
    ) -> Result<Cached<CachedGuildRole>, Error> {
        Ok(CreateGuildRole {
            guild_id: self.get_guild_id(),
            body,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn create_role_with_reason(
        &self,
        ctx: &Context,
        body: CreateGuildRoleBody,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedGuildRole>, Error> {
        Ok(CreateGuildRole {
            guild_id: self.get_guild_id(),
            body,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_role_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRolePositionsEntry>,
    ) -> Result<(), Error> {
        Ok(UpdateGuildRolePositions {
            guild_id: self.get_guild_id(),
            body: positions,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_role_positions_with_reason(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRolePositionsEntry>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error> {
        Ok(UpdateGuildRolePositions {
            guild_id: self.get_guild_id(),
            body: positions,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn reset_role_hoist_positions(&self, ctx: &Context) -> Result<(), Error> {
        // TODO: Caching for this (need to map guild AND role id to GuildRole for this to be possible)
        Ok(ctx
            .get_http_client()
            .execute(ResetGuildRoleHoistPositions {
                guild_id: self.get_guild_id(),
                audit_log_reason: None,
            })
            .await?)
    }

    async fn reset_role_hoist_positions_with_reason(
        &self,
        ctx: &Context,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error> {
        // TODO: Caching for this (need to map guild AND role id to GuildRole for this to be possible)
        Ok(ctx
            .get_http_client()
            .execute(ResetGuildRoleHoistPositions {
                guild_id: self.get_guild_id(),
                audit_log_reason: Some(reason.into()),
            })
            .await?)
    }

    async fn update_role_hoist_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRoleHoistPositionsEntry>,
    ) -> Result<(), Error> {
        Ok(UpdateGuildRoleHoistPositions {
            guild_id: self.get_guild_id(),
            body: positions,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_role_hoist_positions_with_reason(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRoleHoistPositionsEntry>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error> {
        Ok(UpdateGuildRoleHoistPositions {
            guild_id: self.get_guild_id(),
            body: positions,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn delete_role(&self, ctx: &Context, role_id: Id<RoleMarker>) -> Result<(), Error> {
        Ok(DeleteGuildRole {
            guild_id: self.get_guild_id(),
            role_id,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn delete_role_with_reason(
        &self,
        ctx: &Context,
        role_id: Id<RoleMarker>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<(), Error> {
        Ok(DeleteGuildRole {
            guild_id: self.get_guild_id(),
            role_id,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_role(
        &self,
        ctx: &Context,
        role_id: Id<RoleMarker>,
        updates: UpdateGuildRoleBody,
    ) -> Result<Cached<CachedGuildRole>, Error> {
        Ok(UpdateGuildRole {
            guild_id: self.get_guild_id(),
            role_id,
            body: updates,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_role_with_reason(
        &self,
        ctx: &Context,
        role_id: Id<RoleMarker>,
        updates: UpdateGuildRoleBody,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedGuildRole>, Error> {
        Ok(UpdateGuildRole {
            guild_id: self.get_guild_id(),
            role_id,
            body: updates,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn list_stickers(&self, ctx: &Context) -> Result<Vec<GuildSticker>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListGuildStickers {
                guild_id: self.get_guild_id(),
            })
            .await?)
    }

    async fn create_sticker(
        &self,
        ctx: &Context,
        sticker: CreateGuildStickerBody,
    ) -> Result<GuildSticker, Error> {
        Ok(ctx
            .get_http_client()
            .execute(CreateGuildSticker {
                guild_id: self.get_guild_id(),
                body: sticker,
            })
            .await?)
    }

    async fn bulk_create_stickers(
        &self,
        ctx: &Context,
        stickers: Vec<CreateGuildStickerBody>,
    ) -> Result<BulkCreateGuildStickersResponse, Error> {
        Ok(ctx
            .get_http_client()
            .execute(BulkCreateGuildStickers {
                guild_id: self.get_guild_id(),
                body: stickers,
            })
            .await?)
    }

    async fn delete_sticker(
        &self,
        ctx: &Context,
        sticker_id: Id<StickerMarker>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(DeleteGuildSticker {
                guild_id: self.get_guild_id(),
                sticker_id,
            })
            .await?)
    }

    async fn update_sticker(
        &self,
        ctx: &Context,
        sticker_id: Id<StickerMarker>,
        updates: UpdateGuildStickerBody,
    ) -> Result<GuildSticker, Error> {
        Ok(ctx
            .get_http_client()
            .execute(UpdateGuildSticker {
                guild_id: self.get_guild_id(),
                sticker_id,
                body: updates,
            })
            .await?)
    }

    async fn toggle_channel_flexible_names(
        &self,
        ctx: &Context,
        enabled: bool,
    ) -> Result<Cached<Guild>, Error> {
        Ok(ToggleGuildTextChannelFlexibleNames {
            guild_id: self.get_guild_id(),
            enabled,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn toggle_channel_flexible_names_with_reason(
        &self,
        ctx: &Context,
        enabled: bool,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<Guild>, Error> {
        Ok(ToggleGuildTextChannelFlexibleNames {
            guild_id: self.get_guild_id(),
            enabled,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    #[cfg(feature = "user_api")]
    async fn transfer_ownership(
        &self,
        ctx: &Context,
        new_owner_id: Id<UserMarker>,
        auth: SudoVerification,
    ) -> Result<Cached<Guild>, Error> {
        use neptunium_http::endpoints::guild::TransferGuildOwnership;

        Ok(TransferGuildOwnership {
            guild_id: self.get_guild_id(),
            new_owner_id,
            auth,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn get_vanity_url(&self, ctx: &Context) -> Result<GetGuildVanityUrlResponse, Error> {
        Ok(ctx
            .get_http_client()
            .execute(GetGuildVanityUrl {
                guild_id: self.get_guild_id(),
            })
            .await?)
    }

    async fn update_vanity_url(
        &self,
        ctx: &Context,
        code: Option<String>,
    ) -> Result<UpdateGuildVanityUrlResponse, Error> {
        Ok(UpdateGuildVanityUrl {
            guild_id: self.get_guild_id(),
            code,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_vanity_url_with_reason(
        &self,
        ctx: &Context,
        code: Option<String>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<UpdateGuildVanityUrlResponse, Error> {
        Ok(UpdateGuildVanityUrl {
            guild_id: self.get_guild_id(),
            code,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn leave(&self, ctx: &Context) -> Result<(), Error> {
        Ok(LeaveGuild {
            guild_id: self.get_guild_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    #[cfg(feature = "user_api")]
    async fn update_user_settings(
        &self,
        ctx: &Context,
        body: UpdateUserGuildSettingsBody,
    ) -> Result<UserGuildSettings, Error> {
        Ok(ctx
            .get_http_client()
            .execute(UpdateUserGuildSettings {
                guild_id: self.get_guild_id(),
                body,
            })
            .await?)
    }

    async fn update_settings(
        &self,
        ctx: &Context,
        body: UpdateGuildSettingsBody,
    ) -> Result<Cached<Guild>, Error> {
        Ok(UpdateGuildSettings {
            guild_id: self.get_guild_id(),
            body,
            audit_log_reason: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_settings_with_reason(
        &self,
        ctx: &Context,
        body: UpdateGuildSettingsBody,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<Guild>, Error> {
        Ok(UpdateGuildSettings {
            guild_id: self.get_guild_id(),
            body,
            audit_log_reason: Some(reason.into()),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn timeout_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        until: impl Into<Timestamp<Iso8601>> + Send + Sync,
    ) -> Result<Cached<CachedGuildMember>, Error> {
        self.update_member(
            ctx,
            member_id,
            UpdateGuildMemberBody {
                nick: None,
                roles: None,
                mute: None,
                deaf: None,
                communication_disabled_until: Some(Some(until.into())),
                timeout_reason: None,
                voice_channel_id: None,
                voice_connection_id: None,
            },
        )
        .await
    }

    async fn timeout_member_with_reason(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        until: impl Into<Timestamp<Iso8601>> + Send + Sync,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedGuildMember>, Error> {
        self.update_member(
            ctx,
            member_id,
            UpdateGuildMemberBody {
                nick: None,
                roles: None,
                mute: None,
                deaf: None,
                communication_disabled_until: Some(Some(until.into())),
                timeout_reason: Some(reason.into()),
                voice_channel_id: None,
                voice_connection_id: None,
            },
        )
        .await
    }

    async fn untimeout_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
    ) -> Result<Cached<CachedGuildMember>, Error> {
        self.update_member(
            ctx,
            member_id,
            UpdateGuildMemberBody {
                nick: None,
                roles: None,
                mute: None,
                deaf: None,
                communication_disabled_until: Some(None),
                timeout_reason: None,
                voice_channel_id: None,
                voice_connection_id: None,
            },
        )
        .await
    }

    async fn untimeout_member_with_reason(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        reason: impl Into<String> + Send + Sync,
    ) -> Result<Cached<CachedGuildMember>, Error> {
        self.update_member(
            ctx,
            member_id,
            UpdateGuildMemberBody {
                nick: None,
                roles: None,
                mute: None,
                deaf: None,
                communication_disabled_until: Some(None),
                timeout_reason: Some(reason.into()),
                voice_channel_id: None,
                voice_connection_id: None,
            },
        )
        .await
    }
}
