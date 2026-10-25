//! The embed type when sending it to the API ("input").

use crate::{
    misc::HexColor,
    time::timestamp::{Timestamp, representations::Iso8601},
};

/// A rich embed input is an embed a client supplies on a message, and Fluxer always stores it with the `Rich` embed type.
/// The number of embeds on one message is bounded by the resolved `max_embeds_per_message` limit, which defaults to `10`.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct RichEmbed {
    /// Embed destination URL (`1`-`2048` characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub url: Option<String>,
    /// Embed title (Max. `256` characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub color: Option<HexColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub timestamp: Option<Timestamp<Iso8601>>,
    /// `1`-`4096` characters. An empty string is treated as `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthorInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedMediaInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedMediaInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooterInput>,
    /// At most `25` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<EmbedFieldInput>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct EmbedAuthorInput {
    /// `1`-`256` characters.
    #[builder(into)]
    pub name: String,
    /// `1`-`2048` characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub url: Option<String>,
    /// `1`-`2048` characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub icon_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct EmbedMediaInput {
    /// Media URL or attachment reference.
    ///
    /// From the [Fluxer Docs on this field]:
    ///
    /// Either an `http` or `https` URL, or an `attachment://<filename>` reference
    /// naming a `png`, `jpg`, `jpeg`, `webp`, or `gif` attachment on the same message,
    /// and `1`-`2048` characters in either form.
    /// Only image and thumbnail accept the `attachment://` form.
    ///
    /// [Fluxer Docs on this field]: https://docs.fluxer.app/http-api/messages/#embed-media-input-object
    #[builder(into)]
    pub url: Option<String>,
    /// `1`-`4096` characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct EmbedFooterInput {
    /// `1`-`2048` characters.
    #[builder(into)]
    pub text: String,
    /// `1`-`2048` characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub icon_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, bon::Builder)]
pub struct EmbedFieldInput {
    /// Field name, `1`-`256` characters.
    #[builder(into)]
    pub name: String,
    /// Field value, max. `1024` characters.
    #[builder(into)]
    pub value: String,
    /// Whether the field can render inline, defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}
