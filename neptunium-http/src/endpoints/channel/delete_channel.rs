use bon::Builder;
use neptunium_model::id::{Id, marker::ChannelMarker};
#[cfg(feature = "user_api")]
use neptunium_model::user::auth::SudoVerification;
use reqwest::{
    Method,
    header::{HeaderMap, HeaderValue},
};

use crate::{endpoints::Endpoint, request::Request};

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub struct DeleteOrLeaveChannelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_messages: Option<bool>,
}

/// ## Limitations
/// - Deleting a guild channel requires `VIEW_CHANNEL` and `MANAGE_CHANNELS`.
/// - Closing a direct message or leaving a group direct message requires current recipient access.
/// - The personal notes channel cannot be deleted.
/// - Setting delete_messages also requires sudo mode for a user session,
/// satisfied by a valid sudo proof or by the verification fields in the body.
/// A bot token always satisfies it.
#[derive(Builder, Clone, Debug, PartialEq, Eq)]
pub struct DeleteOrLeaveChannel {
    pub channel_id: Id<ChannelMarker>,
    pub params: DeleteOrLeaveChannelParams,
    #[cfg(feature = "user_api")]
    pub body: Option<SudoVerification>,
    pub sudo_mode_jwt: Option<String>,
}

impl Endpoint for DeleteOrLeaveChannel {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        let params = serde_qs::to_string(&self.params).unwrap();

        Request::builder()
            .method(Method::DELETE)
            .maybe_body('blk: {
                #[cfg(feature = "user_api")]
                break 'blk self.body.map(|body| serde_json::to_string(&body).unwrap());
                #[cfg(not(feature = "user_api"))]
                break 'blk None;
            })
            .maybe_headers(self.sudo_mode_jwt.map(|value| {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "X-Fluxer-Sudo-Mode-JWT",
                    HeaderValue::from_str(&value).unwrap(),
                );
                headers
            }))
            .path(format!("/channels/{}?{}", self.channel_id, params))
            .build()
    }
}
