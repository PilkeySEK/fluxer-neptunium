use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, UserMarker},
};
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct RingCallRecipients {
    pub channel_id: Id<ChannelMarker>,
    pub recipients: Option<Vec<Id<UserMarker>>>,
}

impl Endpoint for RingCallRecipients {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        // Omit recipients if they are set to `None`
        let body = if let Some(recipients) = self.recipients {
            json!({
                "recipients": recipients,
            })
        } else {
            json!({})
        };
        Request::builder()
            .method(Method::POST)
            .body(body.to_string())
            .path(format!("/channels/{}/call/ring", self.channel_id))
            .build()
    }
}
