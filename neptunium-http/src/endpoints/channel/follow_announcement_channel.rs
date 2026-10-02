use bon::Builder;
use neptunium_model::{
    channel::FollowedChannel,
    id::{Id, marker::ChannelMarker},
};
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct FollowAnnouncementChannel {
    pub channel_id: Id<ChannelMarker>,
    /// The guild text channel that receives published messages.
    pub webhook_channel_id: Id<ChannelMarker>,
}

impl Endpoint for FollowAnnouncementChannel {
    type Response = FollowedChannel;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(
                json!({
                    "webhook_channel_id": self.webhook_channel_id,
                })
                .to_string(),
            )
            .path(format!("/channels/{}/followers", self.channel_id))
            .build()
    }
}
