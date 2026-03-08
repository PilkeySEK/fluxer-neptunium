//! Types and builders for interacting with the Fluxer HTTP API.

pub const DEFAULT_API_BASE_URL: &str = "https://api.fluxer.app/v1";
pub const DEFAULT_USER_AGENT: &str = "Neptunium-Http";

const VERSION: &str = unwrap_or(option_env!("CARGO_PKG_VERSION"), "unknown");

const fn unwrap_or(option: Option<&'static str>, default: &'static str) -> &'static str {
    if let Some(value) = option {
        value
    } else {
        default
    }
}

pub mod channel;
pub mod client;
