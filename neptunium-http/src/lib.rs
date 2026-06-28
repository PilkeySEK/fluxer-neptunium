//! Types and builders for interacting with the Fluxer HTTP API.

pub const DEFAULT_API_BASE_URL: &str = "https://api.fluxer.app/v1";
/// (name, contact info).
pub const BASE_USER_AGENT: (&str, &str) = ("neptunium-http", "1473229884793090118");

const VERSION: &str = unwrap_or(option_env!("CARGO_PKG_VERSION"), "unknown");

const fn unwrap_or(option: Option<&'static str>, default: &'static str) -> &'static str {
    if let Some(value) = option {
        value
    } else {
        default
    }
}

pub mod client;
pub mod common_types;
pub mod endpoints;
pub mod error;
pub mod file_upload;
#[cfg(feature = "rate-limiting")]
mod ratelimiting;
pub mod request;
// pub mod requests;
// pub mod responses;
