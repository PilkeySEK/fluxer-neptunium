use std::collections::HashMap;

use bon::Builder;
use serde::Serialize;

use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker, UserMarker},
};

#[derive(Serialize, Clone, Debug, Builder)]
#[expect(clippy::type_complexity)]
pub struct GuildSubscriptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Subscribe to member list updates for the specified channel. The two `u32`s are indexes for which
    /// part of the member list we want to receive updates for (e.g. `vec![(0, 99)]` is what the
    /// official client sends initially). The dispatch opcode that the gateway will send is `GUILD_MEMBER_LIST_UPDATE`.
    ///
    /// When scrolling through the member list, make sure to send this with updates indexes.
    ///
    /// Send an empty `Vec` for the channel to stop receiving these updates for the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_list_channels: Option<HashMap<Id<ChannelMarker>, Vec<(u32, u32)>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Id<UserMarker>>>,
    /// If this is set to `true`, the gateway will send a `GUILD_SYNC` event after receiving this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

#[derive(Serialize, Clone, Debug, Builder)]
pub struct LazyRequest {
    pub subscriptions: HashMap<Id<GuildMarker>, GuildSubscriptionRequest>,
}
