use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct AcknowledgeMessagesBulk {
    pub read_states: Vec<(Id<ChannelMarker>, Id<MessageMarker>)>,
}

impl Endpoint for AcknowledgeMessagesBulk {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        #[derive(Serialize)]
        struct AcknowledgeMessagesBulkBodyReadState {
            channel_id: Id<ChannelMarker>,
            message_id: Id<MessageMarker>,
        }
        #[derive(Serialize)]
        struct AcknowledgeMessagesBulkBody {
            read_states: Vec<AcknowledgeMessagesBulkBodyReadState>,
        }

        let body = AcknowledgeMessagesBulkBody {
            read_states: self
                .read_states
                .into_iter()
                .map(|read_state| AcknowledgeMessagesBulkBodyReadState {
                    channel_id: read_state.0,
                    message_id: read_state.1,
                })
                .collect(),
        };

        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&body).unwrap())
            .path("/read-states/ack-bulk".to_owned())
            .build()
    }
}
