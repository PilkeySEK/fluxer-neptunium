use fluxer_model::id::{Id, marker::ChannelMarker};
use zeroize::Zeroizing;

use crate::{DEFAULT_API_BASE_URL, DEFAULT_USER_AGENT, VERSION, client::messages::MessagesClient};

pub mod messages;

#[derive(Debug)]
pub struct HttpClient {
    api_base_url: String,
    token: Zeroizing<String>,
    reqwest_client: reqwest::Client,
    user_agent: String,
}

impl HttpClient {
    #[must_use]
    pub fn new(token: String) -> Self {
        Self {
            api_base_url: DEFAULT_API_BASE_URL.to_owned(),
            reqwest_client: reqwest::Client::default(),
            token: Zeroizing::new(token),
            user_agent: format!("{DEFAULT_USER_AGENT}/{VERSION}"),
        }
    }

    pub fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = user_agent;
    }

    pub fn set_api_base_url(&mut self, url: String) {
        self.api_base_url = url;
    }

    #[must_use]
    pub fn messages(&self, channel_id: Id<ChannelMarker>) -> MessagesClient<'_> {
        MessagesClient {
            channel_id,
            client: self,
        }
    }
}
