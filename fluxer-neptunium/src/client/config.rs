use std::time::Duration;

use bon::Builder;
use neptunium_cache_inmemory::CacheConfig;
use neptunium_http::DEFAULT_API_BASE_URL;

#[derive(Builder, Debug)]
pub struct ClientConfig {
    #[builder(into)]
    pub api_base_url: Option<String>,
    #[builder(default = false)]
    pub always_propagate_event_errors: bool,
    #[builder(default)]
    #[cfg(feature = "user_api")]
    pub token_type: neptunium_http::client::TokenType,
    #[builder(default = true)]
    pub auto_reconnect: bool,
    #[builder(default = Duration::from_secs(30))]
    pub auto_reconnect_wait_time: Duration,
    #[builder(default = CacheConfig::default())]
    pub cache_config: CacheConfig,
    #[builder(default = Duration::from_secs(60))]
    pub connection_process_timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_base_url: Some(DEFAULT_API_BASE_URL.to_owned()),
            always_propagate_event_errors: false,
            #[cfg(feature = "user_api")]
            token_type: neptunium_http::client::TokenType::default(),
            auto_reconnect: true,
            auto_reconnect_wait_time: Duration::from_secs(30),
            cache_config: CacheConfig::default(),
            connection_process_timeout: Duration::from_secs(60),
        }
    }
}
