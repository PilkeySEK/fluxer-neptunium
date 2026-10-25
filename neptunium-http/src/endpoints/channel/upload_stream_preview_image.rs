use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct UploadStreamPreviewImage {
    pub stream_key: String,
    pub channel_id: Id<ChannelMarker>,
    /// Base64-encoded thumbnail image data.
    pub thumbnail: String,
    /// The MIME type of the thumbnail image.
    pub content_type: Option<String>,
}

impl Endpoint for UploadStreamPreviewImage {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        #[derive(Serialize)]
        struct UploadStreamPreviewImageBody {
            pub channel_id: Id<ChannelMarker>,
            pub thumbnail: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub content_type: Option<String>,
        }

        let body = UploadStreamPreviewImageBody {
            channel_id: self.channel_id,
            thumbnail: self.thumbnail,
            content_type: self.content_type,
        };

        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&body).unwrap())
            .path(format!("/streams/{}/preview", self.stream_key))
            .build()
    }
}
