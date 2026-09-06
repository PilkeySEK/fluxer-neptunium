use std::{fmt::Debug, time::Duration};

use bon::Builder;
use neptunium_cache_inmemory::CacheConfig;
use neptunium_http::endpoints::channel::AllowedMentions;
use neptunium_model::gateway::payload::outgoing::PresenceUpdateOutgoing;

use crate::client::ResumeInfo;

#[derive(Builder)]
pub struct ClientConfig {
    #[builder(into)]
    pub api_base_url: Option<String>,
    // #[builder(default = false)]
    // pub always_propagate_event_errors: bool,
    #[builder(default)]
    pub token_type: neptunium_http::client::TokenType,
    // #[builder(default = true)]
    // pub auto_reconnect: bool,
    #[builder(default = CacheConfig::default())]
    pub cache_config: CacheConfig,
    // #[builder(default = Duration::from_secs(60))]
    // pub connection_process_timeout: Duration,
    pub initial_presence: Option<PresenceUpdateOutgoing>,
    #[builder(default = true)]
    pub send_initial_presence_on_every_reconnect: bool,
    #[builder(default = Box::new(|num_tries: usize| {
        const MIN_TIME: Duration = Duration::from_secs(3);
        const MAX_TIME: Duration = Duration::from_mins(1);
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
    }) as Box<dyn Fn(usize) -> Duration + Send + Sync>, into)]
    pub gateway_retry_wait_time_fn: Box<dyn Fn(usize) -> Duration + Send + Sync>,
    /// Add resume info so that the client will try to resume on the first start instead
    /// of creating a new session.
    pub resume_info: Option<ResumeInfo>,
    /// If `allowed_mentions` is not provided when sending a message, this value will be used instead.
    /// The value will only be overwritten when using methods defined in this crate, not in `neptunium-http` or somewhere else.
    /// When sending a request manually using the HTTP client, this will not be applied.
    pub default_allowed_mentions: Option<AllowedMentions>,
    /// This will be sent in the user agent alongside this crate name.
    ///
    /// Example format: `MyBot (+User#0000)` (you should specify contact information as seen here.)
    #[builder(into)]
    pub bot_user_agent_information: Option<String>,
    /// Whether to overwrite the send timeout in the `ShardConfig` when it is set to `None`.
    #[builder(default = true)]
    pub overwrite_send_timeout: bool,
    /// Whether to automatically subscribe to all guild events.
    /// Useful for user bots.
    ///
    /// By default, Fluxer will only send events that would trigger a
    /// notification to users (and user bots), requiring them to
    /// subscribe to guild events. Setting this to `true` will make
    /// fluxer-neptunium subscribe to all events automatically.
    #[cfg(feature = "user_api")]
    #[builder(default = false)]
    pub subscribe_to_everything: bool,
}

impl Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfig")
            .field("api_base_url", &self.api_base_url)
            .field("token_type", &self.token_type)
            .field("cache_config", &self.cache_config)
            .field("initial_presence", &self.initial_presence)
            .field(
                "send_initial_presence_on_every_reconnect",
                &self.send_initial_presence_on_every_reconnect,
            )
            .field("gateway_retry_wait_time_fn", &"...")
            .field("resume_info", &self.resume_info)
            .field("default_allowed_mentions", &self.default_allowed_mentions)
            .field(
                "bot_user_agent_information",
                &self.bot_user_agent_information,
            )
            .field("overwrite_send_timeout", &self.overwrite_send_timeout)
            .field("subscribe_to_everything", &self.subscribe_to_everything)
            .finish()
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}
