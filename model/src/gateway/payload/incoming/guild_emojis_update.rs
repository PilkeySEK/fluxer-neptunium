use serde::{Deserialize, Serialize};

use crate::guild::properties::GuildEmoji;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildEmojisUpdate {
    /// Array of all emojis in the guild.
    pub emojis: Vec<GuildEmoji>,
}
