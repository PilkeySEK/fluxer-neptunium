use async_trait::async_trait;
use neptunium_http::endpoints::{
    guild::{
        channels::{
            create_guild_channel::{CreateGuildChannel, GuildChannelCreateRequest},
            list_guild_channels::ListGuildChannels,
            update_channel_positions::{
                UpdateGuildChannelPositions, UpdateGuildChannelPositionsEntry,
            },
        },
        get_guild_information::GetGuildInformation,
        get_guild_vanity_url::{GetGuildVanityUrl, GetGuildVanityUrlResponse},
        leave_guild::LeaveGuild,
        list_guild_audit_logs::{ListGuildAuditLogs, ListGuildAuditLogsParams},
        list_guild_bans::ListGuildBans,
        members::{
            add_role_to_guild_member::AddRoleToGuildMember,
            ban_guild_member::{BanGuildMember, BanGuildMemberBody},
            get_current_user_guild_member::GetCurrentUserGuildMember,
            get_guild_member::GetGuildMember,
            kick_guild_member::KickGuildMember,
            list_guild_members::ListGuildMembers,
            remove_role_from_member::RemoveRoleFromGuildMember,
            unban_guild_member::UnbanGuildMember,
            update_guild_member::{UpdateGuildMember, UpdateGuildMemberBody},
        },
        roles::{
            create_guild_role::{CreateGuildRole, CreateGuildRoleBody},
            delete_guild_role::DeleteGuildRole,
            list_guild_roles::ListGuildRoles,
            reset_role_hoist_positions::ResetGuildRoleHoistPositions,
            update_guild_role::{UpdateGuildRole, UpdateGuildRoleBody},
            update_role_hoist_positions::{
                UpdateGuildRoleHoistPositions, UpdateGuildRoleHoistPositionsEntry,
            },
            update_role_positions::{UpdateGuildRolePositions, UpdateGuildRolePositionsEntry},
        },
        stickers::{
            bulk_create_guild_stickers::{
                BulkCreateGuildStickers, BulkCreateGuildStickersResponse,
            },
            create_guild_sticker::{CreateGuildSticker, CreateGuildStickerBody},
            delete_guild_sticker::DeleteGuildSticker,
            list_guild_stickers::ListGuildStickers,
            update_guild_sticker::{UpdateGuildSticker, UpdateGuildStickerBody},
        },
        toggle_detached_banner::ToggleDetachedBanner,
        toggle_guild_text_channel_flexible_names::ToggleGuildTextChannelFlexibleNames,
        update_guild_vanity_url::{UpdateGuildVanityUrl, UpdateGuildVanityUrlResponse},
    },
    invites::list_guild_invites::ListGuildInvites,
    webhooks::list_guild_webhooks::ListGuildWebhooks,
};
use neptunium_model::{
    channel::Channel,
    guild::{
        Guild, audit_log::GuildAuditLogs, bans::GuildBanListEntry, member::GuildMember,
        permissions::GuildRole, properties::GuildSticker, webhook::Webhook,
    },
    id::{
        Id,
        marker::{RoleMarker, StickerMarker, UserMarker},
    },
    invites::InviteWithMetadata,
};

use crate::{client::error::Error, events::context::Context, internal::traits::guild::GuildTrait};

#[async_trait]
pub trait GuildExt {
    async fn list_invites(&self, ctx: &Context) -> Result<Vec<InviteWithMetadata>, Error>;
    async fn list_webhooks(&self, ctx: &Context) -> Result<Vec<Webhook>, Error>;
    async fn fetch(&self, ctx: &Context) -> Result<Guild, Error>;
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
    async fn list_channels(&self, ctx: &Context) -> Result<Vec<Channel>, Error>;
    async fn create_channel(
        &self,
        ctx: &Context,
        channel: GuildChannelCreateRequest,
    ) -> Result<Channel, Error>;
    // TODO: Add helper functions for things, such as making a reordering using Vec<Id<ChannelMarker>>
    async fn update_channel_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildChannelPositionsEntry>,
    ) -> Result<(), Error>;
    #[cfg(feature = "user_api")]
    async fn delete(
        &self,
        ctx: &Context,
        auth: neptunium_model::user::auth::SudoVerification,
    ) -> Result<(), Error>;
    async fn toggle_detached_banner(&self, ctx: &Context, detached: bool) -> Result<Guild, Error>;
    /// List the guild members. `limit` defaults to 1 and should not be greater than 1000.
    async fn list_members(
        &self,
        ctx: &Context,
        limit: Option<u16>,
        after: Option<Id<UserMarker>>,
    ) -> Result<Vec<GuildMember>, Error>;
    #[cfg(feature = "user_api")]
    async fn search_members(
        &self,
        ctx: &Context,
        body: neptunium_http::endpoints::guild::members::search_guild_members::SearchGuildMembersBody,
    ) -> Result<
        neptunium_http::endpoints::guild::members::search_guild_members::SearchGuildMembersResponse,
        Error,
    >;
    /// Get the authenticated bot/user as the guild member.
    async fn get_current_member(&self, ctx: &Context) -> Result<GuildMember, Error>;
    #[cfg(feature = "user_api")]
    async fn update_current_member(
        &self,
        ctx: &Context,
        updates: neptunium_http::endpoints::guild::members::update_current_user_guild_member::UpdateCurrentUserGuildMemberBody,
    ) -> Result<GuildMember, Error>;
    async fn get_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
    ) -> Result<GuildMember, Error>;
    async fn kick_member(&self, ctx: &Context, member_id: Id<UserMarker>) -> Result<(), Error>;
    async fn update_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        body: UpdateGuildMemberBody,
    ) -> Result<GuildMember, Error>;
    async fn add_role_to_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<(), Error>;
    async fn remove_role_from_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<(), Error>;
    async fn list_roles(&self, ctx: &Context) -> Result<Vec<GuildRole>, Error>;
    async fn create_role(
        &self,
        ctx: &Context,
        body: CreateGuildRoleBody,
    ) -> Result<GuildRole, Error>;
    async fn update_role_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRolePositionsEntry>,
    ) -> Result<(), Error>;
    async fn reset_role_hoist_positions(&self, ctx: &Context) -> Result<(), Error>;
    async fn update_role_hoist_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRoleHoistPositionsEntry>,
    ) -> Result<(), Error>;
    async fn delete_role(&self, ctx: &Context, role_id: Id<RoleMarker>) -> Result<(), Error>;
    async fn update_role(
        &self,
        ctx: &Context,
        role_id: Id<RoleMarker>,
        updates: UpdateGuildRoleBody,
    ) -> Result<GuildRole, Error>;
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
    ) -> Result<Guild, Error>;
    #[cfg(feature = "user_api")]
    async fn transfer_ownership(
        &self,
        ctx: &Context,
        new_owner_id: Id<UserMarker>,
        password: Option<String>,
    ) -> Result<Guild, Error>;
    async fn get_vanity_url(&self, ctx: &Context) -> Result<GetGuildVanityUrlResponse, Error>;
    async fn update_vanity_url(
        &self,
        ctx: &Context,
        code: Option<String>,
    ) -> Result<UpdateGuildVanityUrlResponse, Error>;
    /// Leave this guild.
    async fn leave(&self, ctx: &Context) -> Result<(), Error>;
}

#[async_trait]
impl<T: GuildTrait> GuildExt for T {
    async fn list_invites(&self, ctx: &Context) -> Result<Vec<InviteWithMetadata>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListGuildInvites {
                guild_id: self.get_guild_id(),
            })
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

    async fn fetch(&self, ctx: &Context) -> Result<Guild, Error> {
        Ok(ctx
            .get_http_client()
            .execute(GetGuildInformation {
                guild_id: self.get_guild_id(),
            })
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
            })
            .await?)
    }

    async fn list_channels(&self, ctx: &Context) -> Result<Vec<Channel>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListGuildChannels {
                guild_id: self.get_guild_id(),
            })
            .await?)
    }

    async fn create_channel(
        &self,
        ctx: &Context,
        channel: GuildChannelCreateRequest,
    ) -> Result<Channel, Error> {
        Ok(ctx
            .get_http_client()
            .execute(CreateGuildChannel {
                guild_id: self.get_guild_id(),
                body: channel,
            })
            .await?)
    }

    async fn update_channel_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildChannelPositionsEntry>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(UpdateGuildChannelPositions {
                guild_id: self.get_guild_id(),
                body: positions,
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn delete(
        &self,
        ctx: &Context,
        auth: neptunium_model::user::auth::SudoVerification,
    ) -> Result<(), Error> {
        use neptunium_http::endpoints::guild::delete_guild::DeleteGuild;

        Ok(ctx
            .get_http_client()
            .execute(DeleteGuild {
                guild_id: self.get_guild_id(),
                auth,
            })
            .await?)
    }

    async fn toggle_detached_banner(&self, ctx: &Context, detached: bool) -> Result<Guild, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ToggleDetachedBanner {
                guild_id: self.get_guild_id(),
                enabled: detached,
            })
            .await?)
    }

    async fn list_members(
        &self,
        ctx: &Context,
        limit: Option<u16>,
        after: Option<Id<UserMarker>>,
    ) -> Result<Vec<GuildMember>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListGuildMembers {
                guild_id: self.get_guild_id(),
                limit,
                after,
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn search_members(
        &self,
        ctx: &Context,
        body: neptunium_http::endpoints::guild::members::search_guild_members::SearchGuildMembersBody,
    ) -> Result<
        neptunium_http::endpoints::guild::members::search_guild_members::SearchGuildMembersResponse,
        Error,
    > {
        Ok(ctx
            .get_http_client()
            .execute(neptunium_http::endpoints::guild::members::search_guild_members::SearchGuildMembers {
                guild_id: self.get_guild_id(),
                body,
            })
            .await?)
    }

    async fn get_current_member(&self, ctx: &Context) -> Result<GuildMember, Error> {
        Ok(ctx
            .get_http_client()
            .execute(GetCurrentUserGuildMember {
                guild_id: self.get_guild_id(),
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn update_current_member(
        &self,
        ctx: &Context,
        updates: neptunium_http::endpoints::guild::members::update_current_user_guild_member::UpdateCurrentUserGuildMemberBody,
    ) -> Result<GuildMember, Error> {
        use neptunium_http::endpoints::guild::members::update_current_user_guild_member::UpdateCurrentUserGuildMember;

        Ok(ctx
            .get_http_client()
            .execute(UpdateCurrentUserGuildMember {
                guild_id: self.get_guild_id(),
                body: updates,
            })
            .await?)
    }

    async fn get_member(
        &self,
        ctx: &Context,
        member_id: Id<UserMarker>,
    ) -> Result<GuildMember, Error> {
        Ok(ctx
            .get_http_client()
            .execute(GetGuildMember {
                guild_id: self.get_guild_id(),
                user_id: member_id,
            })
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
    ) -> Result<GuildMember, Error> {
        Ok(ctx
            .get_http_client()
            .execute(UpdateGuildMember {
                guild_id: self.get_guild_id(),
                user_id: member_id,
                body,
            })
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
            })
            .await?)
    }

    async fn list_roles(&self, ctx: &Context) -> Result<Vec<GuildRole>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListGuildRoles {
                guild_id: self.get_guild_id(),
            })
            .await?)
    }

    async fn create_role(
        &self,
        ctx: &Context,
        body: CreateGuildRoleBody,
    ) -> Result<GuildRole, Error> {
        Ok(ctx
            .get_http_client()
            .execute(CreateGuildRole {
                guild_id: self.get_guild_id(),
                body,
            })
            .await?)
    }

    async fn update_role_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRolePositionsEntry>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(UpdateGuildRolePositions {
                guild_id: self.get_guild_id(),
                body: positions,
            })
            .await?)
    }

    async fn reset_role_hoist_positions(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(ResetGuildRoleHoistPositions {
                guild_id: self.get_guild_id(),
            })
            .await?)
    }

    async fn update_role_hoist_positions(
        &self,
        ctx: &Context,
        positions: Vec<UpdateGuildRoleHoistPositionsEntry>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(UpdateGuildRoleHoistPositions {
                guild_id: self.get_guild_id(),
                body: positions,
            })
            .await?)
    }

    async fn delete_role(&self, ctx: &Context, role_id: Id<RoleMarker>) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(DeleteGuildRole {
                guild_id: self.get_guild_id(),
                role_id,
            })
            .await?)
    }

    async fn update_role(
        &self,
        ctx: &Context,
        role_id: Id<RoleMarker>,
        updates: UpdateGuildRoleBody,
    ) -> Result<GuildRole, Error> {
        Ok(ctx
            .get_http_client()
            .execute(UpdateGuildRole {
                guild_id: self.get_guild_id(),
                role_id,
                body: updates,
            })
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
    ) -> Result<Guild, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ToggleGuildTextChannelFlexibleNames {
                guild_id: self.get_guild_id(),
                enabled,
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn transfer_ownership(
        &self,
        ctx: &Context,
        new_owner_id: Id<UserMarker>,
        password: Option<String>,
    ) -> Result<Guild, Error> {
        use neptunium_http::endpoints::guild::transfer_guild_ownership::TransferGuildOwnership;
        use zeroize::Zeroizing;

        Ok(ctx
            .get_http_client()
            .execute(TransferGuildOwnership {
                guild_id: self.get_guild_id(),
                new_owner_id,
                password: password.map(Zeroizing::new),
            })
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
        Ok(ctx
            .get_http_client()
            .execute(UpdateGuildVanityUrl {
                guild_id: self.get_guild_id(),
                code,
            })
            .await?)
    }

    async fn leave(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(LeaveGuild {
                guild_id: self.get_guild_id(),
            })
            .await?)
    }
}
