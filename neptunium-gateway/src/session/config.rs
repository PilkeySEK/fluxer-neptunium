use std::time::Duration;

use bon::Builder;
use debug_ignore::DebugIgnore;
use neptunium_model::gateway::{intents::GatewayEventFlags, shard::ShardInfo};
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::session::{ResumeInfo, SessionHandle};

#[derive(Debug, Builder)]
pub struct SessionConfig {
    #[builder(default)]
    pub shard_info: ShardInfo,
    #[builder(default = SessionConfig::DEFAULT_GATEWAY_URL.to_owned())]
    pub gateway_base_url: String,
    #[builder(into)]
    pub token: Zeroizing<String>,
    pub ignored_events: Option<GatewayEventFlags>,
    #[builder(default = false)]
    pub force_ipv4: bool,
    /// Timeout for establishing a new connection to the gateway.
    #[builder(default = Duration::from_mins(1))]
    pub connection_timeout: Duration,
    /// Timeout for sending messages to the gateway.
    pub send_timeout: Option<Duration>,
    #[builder(default)]
    pub connection_params: GatewayConnectionParams,
    /// Existing resume info to pass. On the first connect, the session will
    /// try to resume with this info, but discard it if the resuming fails and
    /// reconnect normally instead.
    pub resume_info: Option<ResumeInfo>,
    // /// Called on every `Hello` received. This is likely where you want to send `Identify` or `Resume`.
    // #[builder(into)]
    // pub on_hello:
    //     Option<DebugIgnore<Box<dyn FnMut(SessionHandle) -> Box<dyn Future<Output = ()>>>>>,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub struct GatewayConnectionParams {
    /// This should be `1`.
    #[serde(rename = "v")]
    version: u64,
    #[serde(default)]
    encoding: GatewayEncoding,
    #[serde(with = "gateway_compression", flatten)]
    compression: Option<GatewayCompression>,
}

impl Default for GatewayConnectionParams {
    fn default() -> Self {
        Self {
            version: 1,
            encoding: GatewayEncoding::default(),
            compression: None,
        }
    }
}

mod gateway_compression {
    use serde::{Deserialize, Deserializer, Serializer, de::Visitor, ser::SerializeStruct};

    use crate::session::config::GatewayCompression;

    pub fn serialize<S: Serializer>(
        input: &Option<GatewayCompression>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(input) = input {
            match input {
                GatewayCompression::ZstdStream => {
                    let mut s = serializer.serialize_struct("GatewayCompression", 2)?;
                    s.serialize_field("compress", input)?;
                    s.serialize_field("stream", &true)?;
                    s.end()
                }
            }
        } else {
            let mut s = serializer.serialize_struct("GatewayCompression", 1)?;
            s.serialize_field("compress", &"none")?;
            s.end()
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<GatewayCompression>, D::Error> {
        #[derive(Deserialize)]
        struct CompressionRaw {
            compress: Option<CompressionKindRaw>,
            stream: Option<Stream>,
        }

        struct Stream(bool);

        // Unrecognized values should result in no compression: https://docs.fluxer.app/gateway/overview/#connection-parameters
        impl<'de> Deserialize<'de> for Stream {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct StreamVisitor;

                impl<'de> Visitor<'de> for StreamVisitor {
                    type Value = Stream;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("nothing, 1 or true")
                    }

                    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        // `false` is not a valid value: https://docs.fluxer.app/gateway/overview/#connection-parameters
                        Ok(Stream(v))
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(Stream(match v {
                            "1" | "true" => true,
                            _ => false,
                        }))
                    }

                    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        visit_number_common(v)
                    }
                    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        visit_number_common(v)
                    }
                    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        visit_number_common(v)
                    }
                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        visit_number_common(v)
                    }
                    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        visit_number_common(v)
                    }
                    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        visit_number_common(v)
                    }
                    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        visit_number_common(v)
                    }
                    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        visit_number_common(v)
                    }
                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        visit_number_common(v)
                    }
                    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        visit_number_common(v)
                    }
                }

                fn visit_number_common<N, E>(num: N) -> Result<Stream, E>
                where
                    N: TryInto<u8>,
                    E: serde::de::Error,
                {
                    Ok(Stream(if num.try_into().is_ok_and(|n| n == 1) {
                        true
                    } else {
                        false
                    }))
                }

                deserializer.deserialize_any(StreamVisitor)
            }
        }

        enum CompressionKindRaw {
            ZstdStream,
            None,
        }

        // Unrecognized values should result in no compression: https://docs.fluxer.app/gateway/overview/#connection-parameters
        impl<'de> Deserialize<'de> for CompressionKindRaw {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(if let Ok(string) = String::deserialize(deserializer) {
                    match string.as_str() {
                        "zstd-stream" => Self::ZstdStream,
                        _ => Self::None,
                    }
                } else {
                    Self::None
                })
            }
        }

        let raw = CompressionRaw::deserialize(deserializer)?;
        Ok(match raw.compress {
            Some(CompressionKindRaw::ZstdStream) => {
                if let Some(Stream(true)) = raw.stream {
                    Some(GatewayCompression::ZstdStream)
                } else {
                    None
                }
            }
            Some(CompressionKindRaw::None) | None => None,
        })
    }
}

#[derive(Serialize, Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum GatewayEncoding {
    #[serde(rename = "json")]
    #[default]
    Json,
}

impl<'de> Deserialize<'de> for GatewayEncoding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = String::deserialize(deserializer);
        // Any value selects JSON: https://docs.fluxer.app/gateway/overview/#connection-parameters
        Ok(Self::Json)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum GatewayCompression {
    ZstdStream,
}

impl SessionConfig {
    pub const DEFAULT_GATEWAY_URL: &str = "wss://gateway.fluxer.app/?v=1&encoding=json";
}

impl From<&str> for SessionConfig {
    fn from(value: &str) -> Self {
        Self::builder().token(value.to_owned()).build()
    }
}

impl From<String> for SessionConfig {
    fn from(value: String) -> Self {
        Self::builder().token(value).build()
    }
}

impl From<Zeroizing<String>> for SessionConfig {
    fn from(value: Zeroizing<String>) -> Self {
        Self::builder().token(value).build()
    }
}
