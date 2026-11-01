use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;

use crate::{
    endpoints::{Endpoint, channel::ArchivedThreads},
    request::Request,
};

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct ListJoinedPrivateArchivedThreadsQuery {
    /// Return threads with an ID below this thread ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<Id<ChannelMarker>>,
    /// `2`-`100`, default `50`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct ListJoinedPrivateArchivedThreads {
    pub channel_id: Id<ChannelMarker>,
    pub query: ListJoinedPrivateArchivedThreadsQuery,
}

impl Endpoint for ListJoinedPrivateArchivedThreads {
    type Response = ArchivedThreads;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!(
                "/channels/{}/users/@me/threads/archived/private?{}",
                self.channel_id,
                serde_qs::to_string(&self.query).unwrap()
            ))
            .build()
    }
}
