use std::sync::atomic::{AtomicI64, Ordering};

use time::UtcDateTime;

use crate::time::timestamp::{Timestamp, representations::TimestampRepr};

/// A timestamp that can be updated across threads without any locks. Uses atomic
/// operations. Due to the limitations of atomic types, this can only represent
/// millisecond precision.
pub struct AtomicTimestamp {
    inner: AtomicI64,
}

/// `1 ms = 1_000_000 ns`
const fn millis_to_nanos(millis: i64) -> i128 {
    millis as i128 * 1_000_000
}

impl AtomicTimestamp {
    #[must_use]
    pub fn from_unix_millis(millis: i64) -> Self {
        Self {
            inner: AtomicI64::new(millis),
        }
    }

    #[must_use]
    pub fn load_raw(&self) -> i64 {
        self.inner.load(Ordering::Acquire)
    }

    /// # Panics
    /// Panics if the currently stored unix timestamp is out of range for `UtcDateTime`.
    #[must_use]
    pub fn load_utc_time(&self) -> UtcDateTime {
        UtcDateTime::from_unix_timestamp_nanos(millis_to_nanos(self.load_raw())).unwrap()
    }

    /// # Panics
    /// Panics if the currently stored unix timestamp is out of range for `UtcDateTime`.
    #[must_use]
    pub fn load_timestamp<Repr: TimestampRepr>(&self) -> Timestamp<Repr> {
        Timestamp::from(self.load_utc_time())
    }

    pub fn store_raw(&self, value: i64) {
        self.inner.store(value, Ordering::Release);
    }

    pub fn store_utc_time(&self, value: UtcDateTime) {
        self.store_raw(value.unix_timestamp());
    }

    pub fn store_timestamp<Repr: TimestampRepr>(&self, value: Timestamp<Repr>) {
        self.store_utc_time(UtcDateTime::from(value));
    }

    /// Returns the underlying `i64`. This is safe because
    /// consuming `self` guarantees that no other threads are
    /// concurrently accessing the atomic data.
    pub fn into_inner(self) -> i64 {
        self.inner.into_inner()
    }
}

impl<Repr: TimestampRepr> From<Timestamp<Repr>> for AtomicTimestamp {
    fn from(value: Timestamp<Repr>) -> Self {
        let date_time = UtcDateTime::from(value);
        Self::from_unix_millis(date_time.unix_timestamp())
    }
}

impl<Repr: TimestampRepr> From<AtomicTimestamp> for Timestamp<Repr> {
    fn from(value: AtomicTimestamp) -> Self {
        Self::from(
            UtcDateTime::from_unix_timestamp_nanos(millis_to_nanos(value.inner.into_inner()))
                .unwrap(),
        )
    }
}
