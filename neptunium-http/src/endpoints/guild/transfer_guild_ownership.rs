use bon::Builder;
use neptunium_model::{
    guild::Guild,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
    user::auth::SudoVerification,
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct TransferGuildOwnership {
    pub guild_id: Id<GuildMarker>,
    pub new_owner_id: Id<UserMarker>,
    pub auth: SudoVerification,
}

impl Endpoint for TransferGuildOwnership {
    type Response = Guild;

    fn into_request(self) -> crate::request::Request {
        #[derive(Serialize)]
        struct TransferGuildOwnershipBody {
            new_owner_id: Id<UserMarker>,
            #[serde(flatten)]
            auth: SudoVerification,
        }

        let body = TransferGuildOwnershipBody {
            new_owner_id: self.new_owner_id,
            auth: self.auth,
        };

        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&body).unwrap())
            .path(format!("/guilds/{}/transfer-ownership", self.guild_id))
            .build()
    }
}
