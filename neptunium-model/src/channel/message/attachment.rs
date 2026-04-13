use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::AttachmentMarker},
    serde_bitflags,
    time::{
        duration::{Duration, representation::Seconds},
        timestamp::{Timestamp, representations::Iso8601},
    },
};

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct MessageAttachmentFlags: u32 {
        const IS_SPOILER = 1 << 3;
        const CONTAINS_EXPLICIT_MEDIA = 1 << 4;
        const IS_ANIMATED = 1 << 5;
    }
}

impl Default for MessageAttachmentFlags {
    fn default() -> Self {
        Self::empty()
    }
}

serde_bitflags! {MessageAttachmentFlags, u32}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    /// The MIME type of the attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration<Seconds>>,
    /// Whether the attachment URL has expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp<Iso8601>>,
    /// The name of the attached file.
    pub filename: String,
    pub flags: MessageAttachmentFlags,
    /// The height of the attachment in pixels (for images/videos).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    pub id: Id<AttachmentMarker>,
    #[serde(default)]
    pub nsfw: bool,
    /// The base64 encoded placeholder image for lazy loading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    // Documented as "int32" in the documentation but I want to be safe and use u64 instead.
    /// The size of the attachment in bytes.
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The base64 encoded audio waveform data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform: Option<String>,
    /// The width of the attachment in pixels (for images/videos).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}
