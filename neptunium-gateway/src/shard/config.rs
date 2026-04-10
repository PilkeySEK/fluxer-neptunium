use std::time::Duration;

use bon::Builder;
use neptunium_model::gateway::{intents::GatewayEventFlags, shard::ShardInfo};
use zeroize::Zeroizing;

#[derive(Debug, Builder)]
pub struct ShardConfig {
    #[builder(default)]
    pub shard_info: ShardInfo,
    #[builder(default = ShardConfig::DEFAULT_GATEWAY_URL.to_owned())]
    pub gateway_url: String,
    #[builder(into)]
    pub token: Zeroizing<String>,
    pub ignored_events: Option<GatewayEventFlags>,
    #[builder(default = false)]
    pub force_ipv4: bool,
    /// Timeout for establishing a new connection to the gateway.
    #[builder(default = Duration::from_secs(60))]
    pub connection_timeout: Duration,
}

impl ShardConfig {
    pub const DEFAULT_GATEWAY_URL: &str = "wss://gateway.fluxer.app/?v=1&encoding=json";
}

impl From<&str> for ShardConfig {
    fn from(value: &str) -> Self {
        Self::builder().token(value.to_owned()).build()
    }
}

impl From<String> for ShardConfig {
    fn from(value: String) -> Self {
        Self::builder().token(value).build()
    }
}

// pub struct ShardConfigBuilder {
//     shard_info: Option<ShardInfo>,
//     gateway_url: Option<String>,
//     token: Zeroizing<String>,
//     intents: Intents,
// }
//
// impl ShardConfigBuilder {
//     #[must_use]
//     pub fn new(token: String, intents: Intents) -> Self {
//         Self {
//             shard_info: None,
//             gateway_url: None,
//             token: Zeroizing::new(token),
//             intents,
//         }
//     }
//
//     #[must_use]
//     pub fn gateway_url(mut self, url: impl Into<String>) -> Self {
//         self.gateway_url = Some(url.into());
//         self
//     }
//
//     #[must_use]
//     pub fn shard_info(mut self, info: ShardInfo) -> Self {
//         self.shard_info = Some(info);
//         self
//     }
//
//     #[must_use]
//     pub fn build(self) -> ShardConfig {
//         ShardConfig {
//             shard_info: self.shard_info.unwrap_or(ShardInfo::ONE),
//             gateway_url: self
//                 .gateway_url
//                 .unwrap_or_else(|| ShardConfig::DEFAULT_GATEWAY_URL.to_owned()),
//             token: self.token,
//             intents: self.intents,
//         }
//     }
// }
