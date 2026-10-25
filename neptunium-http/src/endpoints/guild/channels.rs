use bon::Builder;
use neptunium_model::{
    channel::PermissionOverwrite,
    id::{Id, marker::ChannelMarker},
};
use serde::Serialize;

mod create_guild_channel;
mod list_guild_channels;
mod update_channel_positions;

pub use create_guild_channel::*;
pub use list_guild_channels::*;
pub use update_channel_positions::*;

// Source: https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/schema/src/domains/channel/ChannelRequestSchemas.tsx#L63
// TODO: Maybe split this into different channel types
#[derive(Builder, Serialize, Clone, Debug, Default)]
pub struct ChannelRequestBase {
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    /// Voice channel bitrate in bits per second.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    /// Maximum users allowed in a voice channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
}
