use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{EmojiMarker, GuildMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct DeleteGuildEmoji {
    pub guild_id: Id<GuildMarker>,
    pub emoji_id: Id<EmojiMarker>,
    /// This is documented as "string", but it's likely a boolean.
    pub purge: Option<bool>,
}

impl Endpoint for DeleteGuildEmoji {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        let mut path = format!("/guilds/{}/emojis/{}", self.guild_id, self.emoji_id);
        if let Some(purge) = self.purge {
            path = format!("{path}?purge={purge}");
        }
        Request::builder().method(Method::DELETE).path(path).build()
    }
}
