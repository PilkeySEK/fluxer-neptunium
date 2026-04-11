use std::sync::Arc;

#[cfg(feature = "user_api")]
use std::collections::HashMap;

#[cfg(feature = "user_api")]
use neptunium_http::endpoints::{
    channel::PreloadMessagesForChannels,
    guild::{DeleteGuild, TransferGuildOwnership},
    users::{
        GetUserSettings, ListCurrentUserMentions, UpdateCurrentUserProfile, UpdateUserSettings,
    },
};

#[cfg(feature = "user_api")]
use neptunium_model::{
    id::{Id, marker::ChannelMarker},
    user::settings::UserSettings,
};

use async_trait::async_trait;
use neptunium_http::{
    client::HttpClient,
    endpoints::{
        Endpoint, ExecuteEndpointRequestError,
        channel::{
            AddUserToGroupDm, BulkDeleteMessages, CreateMessage, CreatePrivateChannel,
            DeleteChannel, DeleteMessage, DeleteMessageAttachment, DeletePermissionOverwrite,
            EditMessage, FetchMessage, GetChannel, ListChannelMessages, ListPrivateChannels,
            RemoveUserFromGroupDm, SetPermissionOverwrite, UpdateCallRegion, UpdateChannelSettings,
        },
        guild::{
            CreateGuildChannel, CreateGuildRole, DeleteGuildRole, GetGuildInformation, LeaveGuild,
            ListGuildChannels, ListGuildMembers, ListGuildRoles, ToggleDetachedBanner,
            ToggleGuildTextChannelFlexibleNames, UpdateGuildRole, UpdateGuildRoleHoistPositions,
            UpdateGuildRoleHoistPositionsEntry, UpdateGuildRolePositions,
            UpdateGuildRolePositionsEntry, UpdateGuildVanityUrl, UpdateGuildVanityUrlResponse,
        },
        invites::{CreateChannelInvite, ListChannelInvites, ListGuildInvites},
        users::{GetCurrentUserProfile, GetUserById, GetUserProfile},
    },
};
use neptunium_model::{
    channel::PermissionOverwrite,
    guild::{
        Guild,
        permissions::{GuildRole, Permissions},
    },
    invites::InviteWithMetadata,
};

use crate::{
    CachableEndpoint, Cache, Cached, CachedChannel, CachedGuildMember, CachedMessage,
    CachedUserProfileFullResponse, gateway::cached_payload::cache_vec, traits::CacheValue,
};

#[async_trait]
impl CachableEndpoint for GetUserById {
    type Response = Cached<<Self as Endpoint>::Response>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        if let Some(cached_user) = cache.users.get(&self.user_id) {
            return Ok(cached_user);
        }
        let res = client.execute(self).await?;
        let user_id = res.id;
        let user = Cached::new(res);
        cache.users.insert(user_id, user.clone());
        Ok(user)
    }
}

#[async_trait]
impl CachableEndpoint for GetUserProfile {
    type Response = Cached<CachedUserProfileFullResponse>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let cache_key = (self.user_id, self.params.guild_id);
        let cached_profile = cache.user_profiles.get(&cache_key);
        let return_cached_profile = 'blk: {
            let Some(cached_profile) = &cached_profile else {
                break 'blk false;
            };
            let profile = cached_profile.load();
            if self.params.with_mutual_friends && profile.mutual_friends.is_none() {
                false
            } else {
                !(self.params.with_mutual_guilds && profile.mutual_guilds.is_none())
            }
        };
        if return_cached_profile {
            // Will never panic because the code that determines whether to return the cached profile already checks for Some(...)
            return Ok(cached_profile.unwrap());
        }

        let guild_id = self.params.guild_id;
        let res = client.execute(self).await?;
        let mut cached_res =
            CachedUserProfileFullResponse::from_user_profile_full_response(res, guild_id, cache);
        if let Some(cached_profile) = cached_profile {
            {
                let profile = cached_profile.load();
                if cached_res.mutual_friends.is_none()
                    && let Some(mutual_friends) = &profile.mutual_friends
                {
                    let mutual_friends = mutual_friends.clone();
                    cached_res.mutual_friends = Some(mutual_friends);
                }
                if cached_res.mutual_guilds.is_none()
                    && let Some(mutual_guilds) = &profile.mutual_guilds
                {
                    let mutual_guilds = mutual_guilds.clone();
                    cached_res.mutual_guilds = Some(mutual_guilds);
                }
            }
            Ok(cached_profile.store_and_return(cached_res))
        } else {
            let id = cached_res.user.load().id;
            let guild_id = self.params.guild_id;
            let cached = Cached::new(cached_res);
            cache.user_profiles.insert((id, guild_id), cached.clone());
            Ok(cached)
        }
    }
}

#[async_trait]
impl CachableEndpoint for DeleteChannel {
    type Response = <Self as Endpoint>::Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let channel_id = self.channel_id;
        client.execute(self).await?;
        cache.channels.invalidate(&channel_id);
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for GetChannel {
    type Response = Cached<CachedChannel>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        if let Some(cached_channel) = cache.channels.get(&self.channel_id) {
            return Ok(cached_channel);
        }
        let res = client.execute(self).await?;
        Ok(CachedChannel::from_channel(res, cache).insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for UpdateChannelSettings {
    type Response = Cached<CachedChannel>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        Ok(CachedChannel::from_channel(res, cache).insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for UpdateCallRegion {
    type Response = <Self as Endpoint>::Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let channel_id = self.channel_id;
        let region_clone = self.region.clone();
        client.execute(self).await?;
        if let Some(cached_channel) = cache.channels.get(&channel_id) {
            cached_channel.modify(move |channel| channel.rtc_region = Some(region_clone));
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for BulkDeleteMessages {
    type Response = <Self as Endpoint>::Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let messages = self.messages.clone();
        client.execute(self).await?;
        for message in messages {
            cache.messages.invalidate(&message);
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for ListChannelMessages {
    type Response = Vec<Cached<CachedMessage>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_messages = Vec::with_capacity(res.len());
        for message in res {
            cached_messages
                .push(CachedMessage::from_message(message, cache).insert_and_return(cache));
        }
        Ok(cached_messages)
    }
}

#[async_trait]
impl CachableEndpoint for GetCurrentUserProfile {
    type Response = Cached<<Self as Endpoint>::Response>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        Ok(res.insert_and_return(cache))
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for UpdateCurrentUserProfile {
    type Response = Cached<<Self as Endpoint>::Response>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        Ok(res.insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for ListPrivateChannels {
    type Response = Vec<Cached<CachedChannel>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_channels = Vec::with_capacity(res.len());
        for channel in res {
            cached_channels
                .push(CachedChannel::from_channel(channel, cache).insert_and_return(cache));
        }
        Ok(cached_channels)
    }
}

#[async_trait]
impl CachableEndpoint for CreatePrivateChannel {
    type Response = Cached<CachedChannel>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        Ok(
            CachedChannel::from_channel(client.execute(self).await?, cache)
                .insert_and_return(cache),
        )
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for ListCurrentUserMentions {
    type Response = Vec<Cached<CachedMessage>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_messages = Vec::with_capacity(res.len());
        for message in res {
            cached_messages
                .push(CachedMessage::from_message(message, cache).insert_and_return(cache));
        }
        Ok(cached_messages)
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for PreloadMessagesForChannels {
    type Response = HashMap<Id<ChannelMarker>, Cached<CachedMessage>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_messages = HashMap::with_capacity(res.len());
        for (id, message) in res {
            cached_messages.insert(
                id,
                CachedMessage::from_message(message, cache).insert_and_return(cache),
            );
        }
        Ok(cached_messages)
    }
}

#[async_trait]
impl CachableEndpoint for CreateMessage {
    type Response = Cached<CachedMessage>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        Ok(
            CachedMessage::from_message(client.execute(self).await?, cache)
                .insert_and_return(cache),
        )
    }
}

#[async_trait]
impl CachableEndpoint for SetPermissionOverwrite {
    type Response = <Self as Endpoint>::Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        client.execute(self).await?;
        if let Some(existing_channel) = cache.channels.get(&self.channel_id) {
            existing_channel.modify(|channel| {
                if let Some(existing_overwrites) = &mut channel.permission_overwrites {
                    for existing_overwrite in existing_overwrites {
                        if existing_overwrite.id == self.overwrite.id {
                            // https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/api/src/channel/controllers/ChannelController.tsx#L272
                            // Permission overwrites are set to 0 (empty) if they were not provided in the request.
                            existing_overwrite.allow =
                                self.overwrite.allow.unwrap_or(Permissions::empty());
                            existing_overwrite.deny =
                                self.overwrite.deny.unwrap_or(Permissions::empty());
                        }
                    }
                } else {
                    channel.permission_overwrites = Some(vec![PermissionOverwrite {
                        allow: self.overwrite.allow.unwrap_or(Permissions::empty()),
                        deny: self.overwrite.deny.unwrap_or(Permissions::empty()),
                        id: self.overwrite.id,
                        r#type: self.overwrite.r#type,
                    }]);
                }
            });
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for DeletePermissionOverwrite {
    type Response = <Self as Endpoint>::Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        client.execute(self).await?;
        if let Some(existing_channel) = cache.channels.get(&self.channel_id) {
            existing_channel.modify(|channel| {
                if let Some(existing_overwrites) = &mut channel.permission_overwrites {
                    existing_overwrites.retain(|overwrite| overwrite.id != self.overwrite_id);
                }
            });
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for AddUserToGroupDm {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        client.execute(self).await?;
        let Some(cached_user) = cache.users.get(&self.user_id) else {
            // TODO: Maybe spawn a new task to fetch the user, but this might not be a good idea
            return Ok(());
        };
        if let Some(existing_channel) = cache.channels.get(&self.channel_id) {
            existing_channel.modify(|channel| {
                let Some(recipients) = &mut channel.recipients else {
                    tracing::warn!(%self.channel_id, "Cached group DM channel does not have recipients.");
                    return;
                };
                recipients.push(cached_user);
            });
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for RemoveUserFromGroupDm {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        client.execute(self).await?;
        if let Some(existing_channel) = cache.channels.get(&self.channel_id) {
            let _ = existing_channel.try_modify(|channel| {
                let Some(recipients) = &mut channel.recipients else {
                    tracing::warn!(%self.channel_id, "Cached group DM channel does not have recipients.");
                    return Err(());
                };
                let mut index = None;
                for (i, recipient) in recipients.iter().enumerate() {
                    let recipient = recipient.load();
                    if recipient.id == self.user_id {
                        index = Some(i);
                        break;
                    }
                }
                let Some(index) = index else {
                    tracing::trace!("Group DM recipient was not cached.");
                    return Err(());
                };
                recipients.remove(index);
                Ok(())
            });
        }
        Ok(())
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for GetUserSettings {
    type Response = Cached<UserSettings>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        if let Some(cached_settings) = cache.current_user_settings.get() {
            return Ok(cached_settings.clone());
        }
        let res = client.execute(self).await;
        Ok(cache
            .current_user_settings
            .get_or_try_init(|| res.map(Cached::new))?
            .clone())
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for UpdateUserSettings {
    type Response = Cached<UserSettings>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let settings = client.execute(self).await?;
        if let Some(cached_settings) = cache.current_user_settings.get() {
            return Ok(cached_settings.store_and_return(settings));
        }
        Ok(cache
            .current_user_settings
            .get_or_init(|| Cached::new(settings))
            .clone())
    }
}

#[async_trait]
impl CachableEndpoint for CreateChannelInvite {
    type Response = Cached<InviteWithMetadata>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        Ok(client.execute(self).await?.insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for ListChannelInvites {
    type Response = Vec<Cached<InviteWithMetadata>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let invites = client.execute(self).await?;
        let cached_invites = cache_vec!(invites, cache);
        Ok(cached_invites)
    }
}

#[async_trait]
impl CachableEndpoint for ListGuildInvites {
    type Response = Vec<Cached<InviteWithMetadata>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let invites = client.execute(self).await?;
        let cached_invites = cache_vec!(invites, cache);
        Ok(cached_invites)
    }
}

#[async_trait]
impl CachableEndpoint for GetGuildInformation {
    type Response = Cached<Guild>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        if let Some(cached_guild) = cache.guilds.get(&self.guild_id) {
            return Ok(cached_guild);
        }
        let guild = client.execute(self).await?;
        Ok(guild.insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for ListGuildChannels {
    type Response = Vec<Cached<CachedChannel>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let channels = client.execute(self).await?;
        let mut cached_channels = Vec::with_capacity(channels.len());
        for channel in channels {
            cached_channels
                .push(CachedChannel::from_channel(channel, cache).insert_and_return(cache));
        }
        Ok(cached_channels)
    }
}

#[async_trait]
impl CachableEndpoint for CreateGuildChannel {
    type Response = Cached<CachedChannel>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        Ok(CachedChannel::from_channel(res, cache).insert_and_return(cache))
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for DeleteGuild {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let guild_id = self.guild_id;
        client.execute(self).await?;
        cache.guilds.invalidate(&guild_id);
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for ToggleDetachedBanner {
    type Response = Cached<Guild>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let guild = client.execute(self).await?;
        Ok(guild.insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for ListGuildRoles {
    type Response = Vec<Cached<GuildRole>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let roles = client.execute(self).await?;
        let cached_roles = cache_vec!(roles, cache);
        Ok(cached_roles)
    }
}

#[async_trait]
impl CachableEndpoint for CreateGuildRole {
    type Response = Cached<GuildRole>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let role = client.execute(self).await?;
        Ok(role.insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for UpdateGuildRolePositions {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let entries = self.body.clone();
        client.execute(self).await?;
        for UpdateGuildRolePositionsEntry { id, position } in entries {
            if let Some(cached_role) = cache.roles.get(&id) {
                cached_role.modify(|role| role.position = position);
            }
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for UpdateGuildRoleHoistPositions {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let entries = self.body.clone();
        client.execute(self).await?;
        for UpdateGuildRoleHoistPositionsEntry {
            id,
            hoist_position: position,
        } in entries
        {
            if let Some(cached_role) = cache.roles.get(&id) {
                cached_role.modify(|role| role.hoist_position = Some(position));
            }
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for DeleteGuildRole {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let role_id = self.role_id;
        client.execute(self).await?;
        cache.roles.invalidate(&role_id);
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for UpdateGuildRole {
    type Response = Cached<GuildRole>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let role = client.execute(self).await?;
        Ok(role.insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for ToggleGuildTextChannelFlexibleNames {
    type Response = Cached<Guild>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let guild = client.execute(self).await?;
        Ok(guild.insert_and_return(cache))
    }
}

#[cfg(feature = "user_api")]
#[async_trait]
impl CachableEndpoint for TransferGuildOwnership {
    type Response = Cached<Guild>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let guild = client.execute(self).await?;
        Ok(guild.insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for UpdateGuildVanityUrl {
    type Response = UpdateGuildVanityUrlResponse;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let guild_id = self.guild_id;
        let res = client.execute(self).await?;
        let code = res.code.clone();
        if let Some(cached_guild) = cache.guilds.get(&guild_id) {
            cached_guild.modify(|guild| guild.vanity_url_code = Some(code));
        }
        Ok(res)
    }
}

#[async_trait]
impl CachableEndpoint for LeaveGuild {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let guild_id = self.guild_id;
        client.execute(self).await?;
        cache.guilds.invalidate(&guild_id);
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for DeleteMessage {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let message_id = self.message_id;
        client.execute(self).await?;
        cache.messages.invalidate(&message_id);
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for EditMessage {
    type Response = Cached<CachedMessage>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let message = client.execute(self).await?;
        Ok(CachedMessage::from_message(message, cache).insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for FetchMessage {
    type Response = Cached<CachedMessage>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        if let Some(cached_message) = cache.messages.get(&self.message_id) {
            return Ok(cached_message);
        }
        let message = client.execute(self).await?;
        Ok(CachedMessage::from_message(message, cache).insert_and_return(cache))
    }
}

#[async_trait]
impl CachableEndpoint for DeleteMessageAttachment {
    type Response = ();
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let attachment_id = self.attachment_id;
        let message_id = self.message_id;
        client.execute(self).await?;
        if let Some(cached_message) = cache.messages.get(&message_id) {
            let _ = cached_message.try_modify(|message| {
                let Some(attachments) = &mut message.attachments else {
                    return Err(());
                };
                attachments.retain(|attachment| attachment.id != attachment_id);
                Ok(())
            });
        }
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for ListGuildMembers {
    type Response = Vec<Cached<CachedGuildMember>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let guild_id = self.guild_id;
        let res = client.execute(self).await?;
        let res = res
            .into_iter()
            .map(|member| {
                (
                    guild_id,
                    CachedGuildMember::from_guild_member(member, cache),
                )
            })
            .collect::<Vec<_>>();
        Ok(cache_vec!(res, cache))
    }
}
