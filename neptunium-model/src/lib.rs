//! Types and certain helpers, mainly for the Fluxer Gateway API.

pub mod channel;
pub mod gateway;
pub mod guild;
pub mod id;
pub mod invites;
pub mod misc;
pub(crate) mod serde_utils;
pub mod time;
pub mod user;

pub use serde_bool;
