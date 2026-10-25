use serde::{Deserialize, Serialize};

use crate::guild::properties::GuildSticker;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildStickersUpdate {
    pub stickers: Vec<GuildSticker>,
}
