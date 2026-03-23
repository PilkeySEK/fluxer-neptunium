use async_trait::async_trait;
use neptunium_http::endpoints::{
    guild::{
        ban_guild_member::{BanGuildMember, BanGuildMemberBody},
        channels::{
            create_guild_channel::{CreateGuildChannel, GuildChannelCreateRequest},
            list_guild_channels::ListGuildChannels,
            update_channel_positions::{
                UpdateGuildChannelPositions, UpdateGuildChannelPositionsEntry,
            },
        },
        get_guild_information::GetGuildInformation,
        list_guild_audit_logs::{ListGuildAuditLogs, ListGuildAuditLogsParams},
        list_guild_bans::ListGuildBans,
        unban_guild_member::UnbanGuildMember,
    },
    invites::list_guild_invites::ListGuildInvites,
    webhooks::list_guild_webhooks::ListGuildWebhooks,
};
use neptunium_model::{
    channel::Channel,
    guild::{GuildResponse, audit_log::GuildAuditLogs, bans::GuildBanListEntry, webhook::Webhook},
    id::{Id, marker::UserMarker},
    invites::InviteWithMetadata,
};

use crate::{client::error::Error, events::context::Context, internal::traits::guild::GuildTrait};

#[async_trait]
pub trait GuildExt {
    async fn list_invites(&self, ctx: &Context) -> Result<Vec<InviteWithMetadata>, Error>;
    async fn list_webhooks(&self, ctx: &Context) -> Result<Vec<Webhook>, Error>;
    async fn fetch(&self, ctx: &Context) -> Result<GuildResponse, Error>;
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

    async fn fetch(&self, ctx: &Context) -> Result<GuildResponse, Error> {
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
}
