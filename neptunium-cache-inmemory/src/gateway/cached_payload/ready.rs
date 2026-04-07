use std::{collections::HashMap, sync::Arc};

use neptunium_model::{
    gateway::{
        payload::incoming::{
            GuildReadyResponse, Ready, RelationshipReadyResponse, RtcRegion, UserPrivateResponse,
        },
        presence::Presence,
    },
    id::{
        Id,
        marker::{ChannelMarker, UserMarker},
    },
    user::{
        PartialUser,
        read_state::ReadState,
        settings::{FavoriteMeme, UserGuildSettings, UserSettings},
    },
};
use serde_json::Value;
use zeroize::Zeroizing;

use crate::{
    Cache, Cached, CachedChannel,
    gateway::cached_payload::{CachedPayload, cache_option_vec},
    traits::CacheValue,
};

pub struct CachedReady {
    pub version: u64,
    pub session_id: Zeroizing<String>,
    pub user: Cached<UserPrivateResponse>,
    pub guilds: Vec<GuildReadyResponse>,
    pub private_channels: Option<Vec<Cached<CachedChannel>>>,
    pub relationships: Option<Vec<RelationshipReadyResponse>>,
    pub users: Option<Vec<Cached<PartialUser>>>,
    pub presences: Option<Vec<Presence>>,
    pub sessions: Option<Vec<Value>>,
    pub user_settings: Option<Cached<UserSettings>>,
    pub user_guild_settings: Option<Vec<UserGuildSettings>>,
    pub read_states: Option<Vec<ReadState>>,
    pub notes: Option<HashMap<Id<UserMarker>, String>>,
    pub country_code: Option<String>,
    pub pinned_dms: Option<Vec<Id<ChannelMarker>>>,
    pub favorite_memes: Option<Vec<FavoriteMeme>>,
    pub auth_session_id_hash: Option<String>,
    pub rtc_regions: Option<Vec<RtcRegion>>,
    #[cfg(feature = "user_api")]
    pub latitude: Option<String>,
    #[cfg(feature = "user_api")]
    pub longitude: Option<String>,
}

impl CachedPayload for CachedReady {
    type NonCached = Ready;
    fn cache_payload(non_cached: Self::NonCached, cache: &Arc<Cache>) -> Self {
        let user = non_cached.user.insert_and_return(cache);
        let private_channels =
            // cache_option_vec!(non_cached.private_channels, cache, async |value| CachedChannel::from_channel(value, cache).await);
            if let Some(private_channels) = non_cached.private_channels {
                let mut cached_private_channels = Vec::with_capacity(private_channels.len());
                for channel in private_channels {
                    cached_private_channels.push(CachedChannel::from_channel(channel, cache).insert_and_return(cache));
                }
                Some(cached_private_channels)
            } else { None };
        let users = cache_option_vec!(non_cached.users, cache);
        let user_settings = non_cached
            .user_settings
            .map(|settings| settings.insert_and_return(cache));

        Self {
            version: non_cached.version,
            session_id: non_cached.session_id,
            user,
            guilds: non_cached.guilds,
            private_channels,
            relationships: non_cached.relationships,
            users,
            presences: non_cached.presences,
            sessions: non_cached.sessions,
            user_settings,
            user_guild_settings: non_cached.user_guild_settings,
            read_states: non_cached.read_states,
            notes: non_cached.notes,
            country_code: non_cached.country_code,
            pinned_dms: non_cached.pinned_dms,
            favorite_memes: non_cached.favorite_memes,
            auth_session_id_hash: non_cached.auth_session_id_hash,
            rtc_regions: non_cached.rtc_regions,
            #[cfg(feature = "user_api")]
            latitude: non_cached.latitude,
            #[cfg(feature = "user_api")]
            longitude: non_cached.longitude,
        }
    }
}
