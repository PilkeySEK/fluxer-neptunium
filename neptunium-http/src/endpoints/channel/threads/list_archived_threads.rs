use neptunium_model::{
    channel::thread::{ThreadChannel, ThreadMember},
    id::{Id, marker::ChannelMarker},
    time::timestamp::{Timestamp, representations::Iso8601},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct ListArchivedThreadsQuery {
    /// Return threads archived before this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<Timestamp<Iso8601>>,
    /// `2`-`100`, default `50`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct ListArchivedThreads<Vis: ThreadVisibilityMarker> {
    pub channel_id: Id<ChannelMarker>,
    pub query: ListArchivedThreadsQuery,
    pub _vis: std::marker::PhantomData<Vis>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ArchivedThreads {
    /// The archived threads.
    pub threads: Vec<ThreadChannel>,
    /// A thread member object for each returned thread the caller joined.
    pub members: Vec<ThreadMember>,
    /// Whether more threads are available.
    pub has_more: bool,
}

impl<Vis: ThreadVisibilityMarker> Endpoint for ListArchivedThreads<Vis> {
    type Response = ArchivedThreads;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!(
                "/channels/{}/threads/archived/{}?{}",
                self.channel_id,
                Vis::get_path_part(),
                serde_qs::to_string(&self.query).unwrap()
            ))
            .build()
    }
}

pub trait ThreadVisibilityMarker:
    crate::private::Sealed + Copy + Clone + PartialEq + Eq + std::fmt::Debug + Send + Sync
{
    fn get_path_part() -> &'static str;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Private;

impl crate::private::Sealed for Private {}

impl ThreadVisibilityMarker for Private {
    fn get_path_part() -> &'static str {
        "private"
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Public;

impl crate::private::Sealed for Public {}

impl ThreadVisibilityMarker for Public {
    fn get_path_part() -> &'static str {
        "public"
    }
}
