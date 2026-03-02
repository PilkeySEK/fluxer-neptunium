use fluxer_gateway::client::{
    GatewayClient,
    client_config::{GatewayClientConfiguration, GatewayIntents},
};
use zeroize::Zeroizing;

use crate::{Client, EventBus, api::ApiClient, events::EventListener};

const DEFAULT_API_BASE_PATH: &str = "https://api.fluxer.app/v1";
const USER_AGENT: &str = "Fluxer-Neptunium";

pub struct ClientBuilder<'a> {
    gateway_config: GatewayClientConfiguration<'a>,
    api_base_path: &'a str,
    user_agent: String,
    event_bus: EventBus,
}

impl<'a> ClientBuilder<'a> {
    #[must_use]
    pub fn new(intents: GatewayIntents, token: &'a str) -> Self {
        Self {
            gateway_config: GatewayClientConfiguration::new(token, intents),
            api_base_path: DEFAULT_API_BASE_PATH,
            user_agent: USER_AGENT.to_owned(),
            event_bus: EventBus::new(),
        }
    }

    /// Set the API base path. The default is `https://api.fluxer.app/v1`.
    #[must_use]
    pub fn api_base_path(mut self, api_url: &'a str) -> Self {
        self.api_base_path = api_url;
        self
    }

    /// Set the User Agent which is sent in all HTTP requests to the API.
    #[must_use]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Set the gateway url. The default is `wss://gateway.fluxer.app`.
    #[must_use]
    pub fn gateway_url(mut self, gateway_url: &'a str) -> Self {
        self.gateway_config.gateway_url = gateway_url;
        self
    }

    /// Set the gateway version. The default is `1`.
    ///
    /// **NOTE:** Changing this value does not affect how this crate communicates
    /// with the gateway.
    #[must_use]
    pub fn gateway_version(mut self, gateway_version: i32) -> Self {
        self.gateway_config.version = gateway_version;
        self
    }

    #[must_use]
    pub fn with_event_listener(mut self, listener: impl EventListener + Send + 'static) -> Self {
        self.event_bus
            .register(Box::new(listener) as Box<dyn EventListener + Send>);
        self
    }

    #[must_use]
    pub fn build(self) -> crate::Client<'a> {
        let token_clone = self.gateway_config.token.to_owned();
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Client {
            gateway_client: GatewayClient::new(self.gateway_config),
            api_client: ApiClient {
                base_path: self.api_base_path,
                user_agent: format!("{}/{}", USER_AGENT, crate::VERSION),
                token: Zeroizing::new(token_clone),
                reqwest_client: reqwest::Client::new(),
            },
            last_sequence_number: None,
            event_bus: self.event_bus,
            tx,
            rx,
        }
    }
}
