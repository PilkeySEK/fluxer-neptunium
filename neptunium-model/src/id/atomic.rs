use std::{
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::id::{Id, marker::IdMarker};

#[derive(Debug)]
pub struct AtomicId<T: IdMarker> {
    _marker: PhantomData<T>,
    value: AtomicU64,
}

impl<T: IdMarker> Clone for AtomicId<T> {
    fn clone(&self) -> Self {
        Self::new(self.load())
    }
}

impl<T: IdMarker> PartialEq for AtomicId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.load() == other.load()
    }
}

impl<T: IdMarker> Eq for AtomicId<T> {}

impl<T: IdMarker> Hash for AtomicId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.load().hash(state);
    }
}

impl<T: IdMarker> AtomicId<T> {
    /// The Fluxer epoch. Subtract this from a UNIX timestamp (millis) to get the timestamp that should be used inside of a snowflake.
    /// [Source](https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/constants/src/Core.tsx#L20).
    pub const FLUXER_EPOCH: i64 = 1_420_070_400_000;

    /// Create a new ID with the given `value`.
    #[must_use]
    pub fn new(value: u64) -> Self {
        Self {
            value: AtomicU64::new(value),
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub fn load(&self) -> u64 {
        self.value.load(Ordering::Acquire)
    }

    pub fn store_raw(&self, value: u64) {
        self.value.store(value, Ordering::Release);
    }

    pub fn store(&self, value: Id<T>) {
        self.store_raw(value.into_inner());
    }

    /// Cast this ID to a different marker type.
    #[must_use]
    pub fn cast<NewMarker: IdMarker>(self) -> AtomicId<NewMarker> {
        AtomicId::new(self.into_inner())
    }

    /// Consumes the `AtomicId` and returns the underlying value.
    ///
    /// This is safe because passing `self` by value guarantees that
    /// no other threads are concurrently accessing the atomic data.
    #[must_use]
    pub fn into_inner(self) -> u64 {
        self.value.into_inner()
    }
}

impl<T: IdMarker> Display for AtomicId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.load().fmt(f)
    }
}

impl<T: IdMarker> From<Id<T>> for AtomicId<T> {
    fn from(value: Id<T>) -> Self {
        Self::new(value.into_inner())
    }
}

impl<T: IdMarker> From<AtomicId<T>> for Id<T> {
    fn from(value: AtomicId<T>) -> Self {
        Self::new(value.into_inner())
    }
}

impl<T: IdMarker> Serialize for AtomicId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T: IdMarker> Deserialize<'de> for AtomicId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Just use `Id`s implementation, reduces code duplication.
        Ok(Id::deserialize(deserializer)?.into())
    }
}

impl<T: IdMarker> From<OffsetDateTime> for AtomicId<T> {
    fn from(value: OffsetDateTime) -> Self {
        // Just use `Id`s implementation.
        Self::from(Id::from(value))
    }
}
