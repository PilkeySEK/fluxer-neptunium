use neptunium_model::{
    channel::{
        message::Message,
        thread::{ThreadChannel, ThreadMember},
    },
    id::{
        Id,
        marker::{ChannelMarker, ForumTagMarker},
    },
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct SearchThreadsQuery {
    /// Max. `100` characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Max. `20`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<Id<ForumTagMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_setting: Option<SearchThreadsQueryTagSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SearchThreadsQuerySortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SearchThreadsQuerySortOrder>,
    /// `1`-`25`, default `25`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// `0`-`9975`, default `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Return threads with an ID below this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<Id<ChannelMarker>>,
    /// Return threads with an ID above this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_id: Option<Id<ChannelMarker>>,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SearchThreadsQueryTagSetting {
    MatchSome,
    MatchAll,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SearchThreadsQuerySortBy {
    #[default]
    LastMessageTime,
    ArchiveTime,
    Relevance,
    CreationTime,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum SearchThreadsQuerySortOrder {
    Asc,
    #[default]
    Desc,
}

#[derive(Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct SearchThreads {
    pub channel_id: Id<ChannelMarker>,
    pub query: SearchThreadsQuery,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct SearchThreadsResponse {
    pub threads: Vec<ThreadChannel>,
    pub members: Vec<ThreadMember>,
    pub has_more: bool,
    pub total_results: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_messages: Option<Vec<Message>>,
}

impl Endpoint for SearchThreads {
    type Response = SearchThreadsResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!(
                "/channels/{}/threads/search?{}",
                self.channel_id,
                serde_qs::to_string(&self.query).unwrap()
            ))
            .build()
    }
}
