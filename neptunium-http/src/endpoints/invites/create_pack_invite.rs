use bon::Builder;
use neptunium_model::{
    invites::InviteWithMetadata,
    time::duration::{Duration, Seconds},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct CreatePackInvite {
    #[builder(into)]
    pub pack_id: String,
    /// 0-100, maximum uses of this invite. 0 means unlimited.
    pub max_uses: Option<u8>,
    /// 0-604800, duration until the invite expires. 0 means never.
    pub max_age: Option<Duration<Seconds>>,
    /// Whether to create a new invite or reuse an existing one.
    pub unique: Option<bool>,
}

impl Endpoint for CreatePackInvite {
    type Response = InviteWithMetadata;

    fn into_request(self) -> crate::request::Request {
        #[derive(Serialize)]
        struct CreatePackInviteBody {
            #[serde(skip_serializing_if = "Option::is_none")]
            max_uses: Option<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_age: Option<Duration<Seconds>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            unique: Option<bool>,
        }

        let body = CreatePackInviteBody {
            max_uses: self.max_uses,
            max_age: self.max_age,
            unique: self.unique,
        };

        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&body).unwrap())
            .path(format!("/packs/{}/invites", self.pack_id))
            .build()
    }
}
