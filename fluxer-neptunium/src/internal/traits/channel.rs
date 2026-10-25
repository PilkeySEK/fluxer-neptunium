use neptunium_cache_inmemory::CachedChannel;
use neptunium_model::{
    channel::{Channel, ChannelPartial},
    id::{Id, marker::ChannelMarker},
};

/// Things that all channels have in common.
pub trait ChannelTrait: Sync + Send {
    fn get_channel_id(&self) -> Id<ChannelMarker>;
}

impl ChannelTrait for Channel {
    fn get_channel_id(&self) -> Id<ChannelMarker> {
        self.id
    }
}

impl ChannelTrait for ChannelPartial {
    fn get_channel_id(&self) -> Id<ChannelMarker> {
        self.id
    }
}

impl ChannelTrait for Id<ChannelMarker> {
    fn get_channel_id(&self) -> Id<ChannelMarker> {
        *self
    }
}

impl ChannelTrait for CachedChannel {
    fn get_channel_id(&self) -> Id<ChannelMarker> {
        self.id
    }
}
