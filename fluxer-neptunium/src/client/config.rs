use std::{fmt::Debug, time::Duration};

use bon::Builder;
use neptunium_cache_inmemory::CacheConfig;
use neptunium_model::gateway::payload::outgoing::PresenceUpdateOutgoing;

#[derive(Builder)]
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
    #[builder(default = CacheConfig::default())]
    pub cache_config: CacheConfig,
    #[builder(default = Duration::from_secs(60))]
    pub connection_process_timeout: Duration,
    pub initial_presence: Option<PresenceUpdateOutgoing>,
    #[builder(default = true)]
    pub send_initial_presence_on_every_reconnect: bool,
    #[builder(default = Box::new(|num_tries| {
        const MIN_TIME: Duration = Duration::from_secs(3);
        const MAX_TIME: Duration = Duration::from_secs(60);
        const BASE: f64 = 2.0;
        #[expect(clippy::cast_precision_loss)]
        let time = Duration::from_secs_f64(BASE.powf(num_tries as f64));
        if time < MIN_TIME {
            MIN_TIME
        } else if time > MAX_TIME {
            MAX_TIME
        } else {
            time
        }
    }))]
    pub gateway_retry_wait_time_fn: Box<dyn Fn(usize) -> Duration>,
    /// Override the heartbeat interval (ignore the heartbeat interval requested by the gateway).
    pub heartbeat_interval_override: Option<Duration>,
}

impl Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ClientConfig { ")?;
        f.write_fmt(format_args!("api_base_url: {:?}, ", self.api_base_url))?;
        f.write_fmt(format_args!(
            "always_propagate_event_errors: {:?}, ",
            self.always_propagate_event_errors
        ))?;
        #[cfg(feature = "user_api")]
        f.write_fmt(format_args!("token_type: {:?}, ", self.token_type))?;
        f.write_fmt(format_args!("auto_reconnect: {:?}, ", self.auto_reconnect))?;
        f.write_fmt(format_args!("cache_config: {:?}, ", self.cache_config))?;
        f.write_fmt(format_args!(
            "connection_process_timeout: {:?}, ",
            self.connection_process_timeout
        ))?;
        f.write_fmt(format_args!(
            "initial_presence: {:?}, ",
            self.initial_presence
        ))?;
        f.write_fmt(format_args!(
            "send_initial_presence_on_every_reconnect: {:?}, ",
            self.send_initial_presence_on_every_reconnect
        ))?;
        f.write_str("gateway_retry_wait_time_fn: <closure> }, ")?;
        f.write_fmt(format_args!(
            "heartbeat_interval_override: {:?} ",
            self.heartbeat_interval_override
        ))?;
        Ok(())
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}
