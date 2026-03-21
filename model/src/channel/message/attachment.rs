use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::AttachmentMarker},
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

impl Serialize for MessageAttachmentFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.bits().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MessageAttachmentFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_bits_truncate(u32::deserialize(deserializer)?))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_hash: Option<String>,
    /// The MIME type of the attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Duration<Seconds>>,
    /// Whether the attachment URL has expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    expired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<Timestamp<Iso8601>>,
    /// The name of the attached file.
    filename: String,
    flags: MessageAttachmentFlags,
    /// The height of the attachment in pixels (for images/videos).
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
    id: Id<AttachmentMarker>,
    #[serde(default)]
    nsfw: bool,
    /// The base64 encoded placeholder image for lazy loading.
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_url: Option<String>,
    // Documented as "int32" in the documentation but I want to be safe and use u64 instead.
    /// The size of the attachment in bytes.
    size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    /// The base64 encoded audio waveform data.
    #[serde(skip_serializing_if = "Option::is_none")]
    waveform: Option<String>,
    /// The width of the attachment in pixels (for images/videos).
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
}
