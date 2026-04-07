mod guild_create;
mod guild_role_payloads;
mod message_events;
mod ready;

use std::sync::Arc;

pub use guild_create::*;
pub use guild_role_payloads::*;
pub use message_events::*;
pub use ready::*;

use crate::Cache;

pub(crate) trait CachedPayload {
    type NonCached;
    fn cache_payload(non_cached: Self::NonCached, cache: &Arc<Cache>) -> Self;
}

macro_rules! cache_vec {
    ($input:expr, $cache:expr) => {{
        let mut cached_vec = Vec::with_capacity($input.len());
        for elem in $input {
            cached_vec.push($crate::traits::CacheValue::insert_and_return(elem, $cache));
        }
        cached_vec
    }};
}

pub(crate) use cache_vec;

macro_rules! cache_option_vec {
    ($input:expr, $cache:expr) => {{
        if let Some(some_input) = $input {
            Some($crate::gateway::cached_payload::cache_vec!(
                some_input, $cache
            ))
        } else {
            None
        }
    }};
}

pub(crate) use cache_option_vec;
