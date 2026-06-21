use bon::Builder;
use neptunium_model::{
    id::{Id, marker::ChannelMarker},
    time::{
        duration::{Duration, Millis, Seconds},
        timestamp::{Timestamp, representations::Iso8601},
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

/// Used by the official Fluxer client to determine when it may send the next message.
#[derive(Copy, Clone, Debug, Builder)]
pub struct GetChannelSlowmodeInformation {
    pub channel_id: Id<ChannelMarker>,
}

/// Slowmode information for this channel and user. This includes information
/// about when the client may send the next message too.
#[derive(Deserialize, Copy, Clone, Debug, Serialize)]
pub struct ChannelSlowmodeInformation {
    /// Whether the current user can bypass slowmode.
    pub can_bypass: bool,
    /// When the next message can be sent. `None` if the slowmode
    /// duration has expired for the user or if no slowmode is set
    /// in the channel.
    pub next_send_allowed_at: Option<Timestamp<Iso8601>>,
    /// The slowmode duration. 0 if no slowmode is set.
    pub rate_limit_per_user: Duration<Seconds>,
    /// When the current user should retry sending a message.
    /// (0 if the slowmode for the user has already expired or no
    /// slowmode is set in the channel.)
    #[serde(rename = "retry_after_ms")]
    pub retry_after: Duration<Millis>,
}

impl Endpoint for GetChannelSlowmodeInformation {
    type Response = ChannelSlowmodeInformation;
    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/channels/{}/slowmode", self.channel_id))
            .build()
    }
}
