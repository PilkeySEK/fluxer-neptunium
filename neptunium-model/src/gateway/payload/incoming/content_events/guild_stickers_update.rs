use serde::Deserialize;

use crate::guild::properties::GuildSticker;

#[derive(Deserialize, Clone, Debug)]
pub struct GuildStickersUpdate {
    pub stickers: Vec<GuildSticker>,
}
