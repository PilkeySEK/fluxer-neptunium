use neptunium_model::{
    channel::thread::{ThreadAutoArchiveDuration, ThreadChannel, ThreadFlags},
    id::{
        Id,
        marker::{ChannelMarker, ForumTagMarker},
    },
    time::duration::{Duration, Seconds},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

/// # Limitations
/// - `name`, `auto_archive_duration`, and archiving need the thread owner or a thread moderator.
/// - `locked` needs a thread moderator, and the thread owner can set it to true.
/// - `rate_limit_per_user` and `flags` need a thread moderator.
/// - `invitable` needs the thread owner or a thread moderator.
/// - Unarchiving needs a thread member, the thread owner, or a thread moderator, and needs a thread moderator when the thread is locked.
/// - An archived thread accepts no other change unless the same body sets `archived` to false.
#[derive(bon::Builder, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModifyThreadBody {
    /// `1`-`100` characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// `Some(None)` sets `0`. `0`-`21600`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<Option<Duration<Seconds>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ThreadFlags>,
    /// The complete list of forum tag IDs (max. `5`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Id<ForumTagMarker>>,
    /// Applies only to a private thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
}

/// ## Limitations for the `body`
/// - `name`, `auto_archive_duration`, and archiving need the thread owner or a thread moderator.
/// - `locked` needs a thread moderator, and the thread owner can set it to true.
/// - `rate_limit_per_user` and `flags` need a thread moderator.
/// - `invitable` needs the thread owner or a thread moderator.
/// - Unarchiving needs a thread member, the thread owner, or a thread moderator, and needs a thread moderator when the thread is locked.
/// - An archived thread accepts no other change unless the same body sets `archived` to false.
#[derive(bon::Builder, Clone, Debug, PartialEq, Eq)]
pub struct ModifyThread {
    pub thread_id: Id<ChannelMarker>,
    pub body: ModifyThreadBody,
}

impl Endpoint for ModifyThread {
    type Response = ThreadChannel;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/channels/{}", self.thread_id))
            .build()
    }
}
