use neptunium_model::{
    gateway::payload::incoming::GuildReadyResponse,
    guild::Guild,
    id::{Id, marker::GuildMarker},
};

pub trait GuildTrait: Sync + Send + 'static {
    fn get_guild_id(&self) -> Id<GuildMarker>;
}

impl GuildTrait for Guild {
    fn get_guild_id(&self) -> Id<GuildMarker> {
        self.id
    }
}

impl GuildTrait for GuildReadyResponse {
    fn get_guild_id(&self) -> Id<GuildMarker> {
        self.id
    }
}

impl GuildTrait for Id<GuildMarker> {
    fn get_guild_id(&self) -> Id<GuildMarker> {
        *self
    }
}
