use crate::{
    misc::{HexColor, serde_bitflags},
    time::{
        duration::{Duration, Seconds},
        timestamp::{Timestamp, representations::Iso8601},
    },
};

mod rich;
use bitflags::bitflags;
pub use rich::*;

/// An embed received from the API.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Embed {
    #[serde(rename = "type")]
    pub r#type: EmbedType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp<Iso8601>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<HexColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    /// Produced only by the unfurler when it resolves a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedMedia>,
    /// Produced only by the unfurler when it resolves a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedMedia>,
    /// Produced only by the unfurler when it resolves a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<EmbedMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<EmbedField>>,
    /// Whether the embed contains explicit media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Sanitised oEmbed markup for a trusted specialized renderer.
    /// Produced only by the unfurler when it resolves a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Preferred pixel width of the sanitised oEmbed markup.
    /// Produced only by the unfurler when it resolves a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_width: Option<u64>,
    /// Preferred pixel height of the sanitised oEmbed markup.
    /// Produced only by the unfurler when it resolves a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_height: Option<u64>,
    /// At most one nested embed which itself has no `children` field.
    /// Produced only by the unfurler when it resolves a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Embed>>,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EmbedType {
    /// An embed the unfurler built itself, and the type of every accepted rich embed input.
    Rich,
    /// A generic link preview with page metadata and no resolved media.
    Link,
    /// An image preview resolved from a linked URL.
    Image,
    /// A video preview resolved from a linked URL.
    Video,
    /// An audio preview resolved from a linked URL.
    Audio,
    /// An animated looping video preview.
    Gifv,
    /// An article or encyclopaedia page preview.
    Article,
    /// A Bluesky post preview.
    Bluesky,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmbedAuthor {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmbedMedia {
    pub url: String,
    pub proxy_url: String,
    /// Pixel width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    /// Pixel height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration<Seconds>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    /// Encoded media placeholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MediaFlags>,
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct MediaFlags: u8 {
        const CONTAINS_EXPLICIT_MEDIA = 1 << 4;
        const IS_ANIMATED = 1 << 5;
    }
}

serde_bitflags!(MediaFlags, u8);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmbedFooter {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmbedProvider {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
