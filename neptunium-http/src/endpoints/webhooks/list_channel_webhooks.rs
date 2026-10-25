use bon::Builder;
use neptunium_model::{
    guild::webhook::Webhook,
    id::{Id, marker::ChannelMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListChannelWebhooks {
    pub channel_id: Id<ChannelMarker>,
}

impl Endpoint for ListChannelWebhooks {
    type Response = Vec<Webhook>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/channels/{}/webhooks", self.channel_id))
            .build()
    }
}
