use bon::Builder;
use neptunium_model::{
    id::{
        Id,
        marker::{ChannelMarker, ScheduledMessageMarker},
    },
    time::timestamp::{Timestamp, representations::Iso8601},
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    endpoints::{
        Endpoint,
        channel::{ScheduleMessageResponsePayload, ScheduledMessageStatus},
    },
    request::Request,
};

#[derive(Builder, Copy, Clone, Debug)]
pub struct GetScheduledMessage {
    pub scheduled_message_id: Id<ScheduledMessageMarker>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ScheduledMessageResponse {
    pub id: Id<ScheduledMessageMarker>,
    pub channel_id: Id<ChannelMarker>,
    pub scheduled_at: Timestamp<Iso8601>,
    pub scheduled_local_at: Timestamp<Iso8601>,
    pub timezone: String,
    pub status: ScheduledMessageStatus,
    pub status_reason: Option<String>,
    pub payload: ScheduleMessageResponsePayload,
    pub created_at: Timestamp<Iso8601>,
    pub invalidated_at: Option<Timestamp<Iso8601>>,
}

impl Endpoint for GetScheduledMessage {
    type Response = ScheduledMessageResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!(
                "/users/@me/scheduled-messages/{}",
                self.scheduled_message_id
            ))
            .build()
    }
}
