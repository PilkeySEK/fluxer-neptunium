use std::time::SystemTime;

use serde::{Deserialize, Serialize};

/// A client-provided value for message deduplication. This should be unique for every message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct Nonce(pub String);

impl Nonce {
    /// Creates a new nonce based on the current system time in nanoseconds.
    /// Assuming only this method is used to create `Nonce`s and a new message is not created faster than
    /// one per nanosecond, this will create unique `Nonce`s.
    #[expect(clippy::missing_panics_doc)]
    #[must_use]
    pub fn generate() -> Self {
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time went backwards");
        Self(duration.as_nanos().to_string())
    }

    /// Crates a new nonce with the specified value.
    /// For generating nonces, you likely want to use [`Nonce::generate`].
    #[must_use]
    pub fn new(value: String) -> Self {
        Self(value)
    }
}
