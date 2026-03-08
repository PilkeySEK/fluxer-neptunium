use std::ops::{Deref, DerefMut};

use fluxer_model::{
    channel::message::attachment::MessageAttachmentFlags,
    id::{Id, marker::GenericMarker},
    time::duration::{Duration, representation::Seconds},
};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct AttachmentBase {
    /// 1-1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 1-4096 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub flags: MessageAttachmentFlags,
    /// Duration of the audio file in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration<Seconds>>,
    /// Base64 encoded audio waveform data (1-4096 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AttachmentRequest {
    #[serde(flatten)]
    pub base: AttachmentBase,
    pub id: Id<GenericMarker>,
    /// The name of the file being uploaded (1-255 characters).
    pub filename: String,
}

impl Deref for AttachmentRequest {
    type Target = AttachmentBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for AttachmentRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
