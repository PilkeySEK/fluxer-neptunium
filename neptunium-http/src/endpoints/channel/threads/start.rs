use neptunium_model::{
    channel::{
        message::{MessageFlags, attachment::PreUploadedAttachment, embed::RichEmbed},
        thread::{ForumPost, ThreadAutoArchiveDuration, ThreadChannel, ThreadType},
    },
    id::{
        Id,
        marker::{ChannelMarker, ForumTagMarker, StickerMarker},
    },
};
use reqwest::Method;
use serde::Serialize;
use serde_constant::ConstU8;

use crate::{
    endpoints::{Endpoint, ResponseBody, channel::AllowedMentions},
    private::Sealed,
    request::Request,
};

pub trait StartThreadBody: Sealed + Serialize + std::fmt::Debug + PartialEq + Eq + Clone {
    type Response: ResponseBody;
}

/// Starts a thread with no source message, or creates a post in a forum or media channel.
#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct StartThread<B: StartThreadBody> {
    pub channel_id: Id<ChannelMarker>,
    pub body: B,
}

/// The body for [`StartThread`] in a text or announcement channel.
///
/// [`StartThread`]: StartThread
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct StartThreadTextChannelBody {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: ThreadType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    /// The slowmode interval in seconds, `0`-`21600`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
    /// Whether a member who is not a thread moderator can add members
    /// who are not thread moderators (default true).
    /// Applies only to a private thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
}

/// The body for [`StartThread`] in a forum or media channel.
///
/// [`StartThread`]: StartThread
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct StartThreadForumChannelBody {
    pub name: String,
    pub message: ForumChannelFirstMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<Id<ForumTagMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    /// The slowmode interval in seconds, `0`-`21600`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ConstU8<11>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct ForumChannelFirstMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<RichEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<PreUploadedAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_ids: Option<Vec<Id<StickerMarker>>>,
}

impl crate::private::Sealed for StartThreadTextChannelBody {}
impl crate::private::Sealed for StartThreadForumChannelBody {}

impl StartThreadBody for StartThreadTextChannelBody {
    type Response = ThreadChannel;
}

impl StartThreadBody for StartThreadForumChannelBody {
    type Response = ForumPost;
}

impl<B: StartThreadBody> Endpoint for StartThread<B> {
    type Response = B::Response;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self.body).unwrap())
            .path(format!("/channels/{}/threads", self.channel_id))
            .build()
    }
}
