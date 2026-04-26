// TODO: What about self hosted instances which don't have access to s3 thing?

use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug, Serialize)]
pub struct CreateAttachmentsInChannelAttachment {
    // NOTE: This technically accepts a string too.
    /// 0 is the first attachment, 1 the second, etc.
    pub id: u64,
    #[builder(into)]
    pub filename: String,
    pub file_size: usize,
    /// Is usually `application/octet-stream`.
    #[builder(into, default = "application/octet-stream".to_owned())]
    pub content_type: String,
}

#[derive(Builder, Clone, Debug)]
pub struct CreateAttachmentsInChannel {
    pub attachments: Vec<CreateAttachmentsInChannelAttachment>,
    pub channel_id: Id<ChannelMarker>,
}

/// The API echoes `id`, `filename`, `file_size` and `content_type`.
#[derive(Clone, Debug, Deserialize)]
pub struct CreateAttachmentsInChannelAttachmentResponse {
    pub id: u64,
    pub filename: String,
    pub file_size: usize,
    pub upload_filename: String,
    pub content_type: String,
    // TODO: Which values are possible?
    /// I have observed this to always "singlepart", but can't 100% confirm.
    pub upload_mode: String,
    /// URL to where the file can be uploaded using the S3 API.
    pub upload_url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateAttachmentsInChannelResponse {
    pub attachments: Vec<CreateAttachmentsInChannelAttachmentResponse>,
}

impl Endpoint for CreateAttachmentsInChannel {
    type Response = CreateAttachmentsInChannelResponse;

    fn into_request(self) -> crate::request::Request {
        let body = json!({
            "attachments": self.attachments,
        })
        .to_string();

        Request::builder()
            .method(Method::POST)
            .body(body)
            .path(format!("/channels/{}/attachments", self.channel_id))
            .build()
    }
}
