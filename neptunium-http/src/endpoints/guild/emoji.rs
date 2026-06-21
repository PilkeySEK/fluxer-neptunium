use bon::Builder;
use serde::{Deserialize, Serialize};

mod bulk_create_guild_emojis;
mod create_guild_emoji;
mod delete_guild_emoji;
mod list_guild_emojis;
mod update_guild_emoji;

pub use bulk_create_guild_emojis::*;
pub use create_guild_emoji::*;
pub use delete_guild_emoji::*;
pub use list_guild_emojis::*;
pub use update_guild_emoji::*;

#[derive(Builder, Clone, Debug, Serialize)]
pub struct GuildEmojiCreateData {
    /// The emoji name. 2-32 characters, alphanumeric and underscores only.
    pub name: String,
    /// Base64 encoded image data.
    pub image: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct EmojiCreateFailure {
    pub name: String,
    pub error: String,
}
