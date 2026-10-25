use neptunium_model::{
    channel::Channel,
    id::{Id, marker::UserMarker},
};
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Clone, Debug)]
pub enum CreatePrivateChannel {
    /// A DM with only one recipient (not a group DM).
    Dm(Id<UserMarker>),
    /// A group DM with up to 24 other recipients.
    GroupDm(Vec<Id<UserMarker>>),
}

impl Endpoint for CreatePrivateChannel {
    type Response = Channel;

    fn into_request(self) -> crate::request::Request {
        let body = match self {
            Self::Dm(recipient) => json!({
                "recipient_id": recipient,
            }),
            Self::GroupDm(recipients) => json!({
                "recipients": recipients,
            }),
        };

        Request::builder()
            .method(Method::POST)
            .body(body.to_string())
            .path("/users/@me/channels".to_owned())
            .build()
    }
}
