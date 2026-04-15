use bon::Builder;
use neptunium_model::{
    id::{Id, marker::ChannelMarker},
    invites::InviteWithMetadata,
    time::duration::{Duration, Seconds},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Copy, Clone, Debug, Builder, Default)]
pub struct CreateChannelInviteOptions {
    /// 0-100, where 0 means unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u8>,
    /// 0-604800, duration until the invite expires where 0 means never.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<Duration<Seconds>>,
    /// Whether to create a unique invite or reuse an existing one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    // TODO: Probably say that this only applies to voice channels?
    /// Whether members that joined via this invite should be kicked after disconnecting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct CreateChannelInvite {
    pub channel_id: Id<ChannelMarker>,
    pub options: CreateChannelInviteOptions,
}

impl Endpoint for CreateChannelInvite {
    type Response = InviteWithMetadata;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.options).unwrap())
            .path(format!("/channels/{}/invites", self.channel_id))
            .build()
    }
}
