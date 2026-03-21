use std::{fmt::Display, marker::PhantomData};

use serde::{
    Deserialize, Serialize,
    de::{Unexpected, Visitor},
};
use time::OffsetDateTime;

use crate::id::marker::IdMarker;

pub mod marker;

/// "Snowflake" is a format for uniquely identifiable descriptors (IDs). These IDs are guaranteed to be unique across all of Fluxer, except
/// in some unique scenarios in which child objects share their parent's ID. Snowflakes are always returned as a String in the HTTP and Gateway API,
/// but are stored internally as a `u64` in this struct.
/// This struct represents a Snowflake/ID. The generic parameter is for ensuring that, for example, only an ID with `UserMarker` can be used where a
/// user ID is required.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Id<T: IdMarker> {
    _marker: core::marker::PhantomData<T>,
    value: u64,
}

impl<T: IdMarker> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: IdMarker> Id<T> {
    /// The Fluxer epoch. Subtract this from a UNIX timestamp (millis) to get the timestamp that should be used inside of a snowflake.
    /// [Source](https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/constants/src/Core.tsx#L20)
    pub const FLUXER_EPOCH: i64 = 1_420_070_400_000;
    /// Create a new ID with the given `value`.
    #[must_use]
    pub fn new(value: u64) -> Self {
        Self {
            value,
            _marker: core::marker::PhantomData,
        }
    }

    /// Cast this ID to a different marker type.
    #[must_use]
    pub fn cast<NewMarker: IdMarker>(self) -> Id<NewMarker> {
        Id::new(self.value)
    }
}

impl<T: IdMarker> From<u64> for Id<T> {
    fn from(value: u64) -> Self {
        Self {
            value,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<T: IdMarker> From<Id<T>> for u64 {
    fn from(value: Id<T>) -> Self {
        value.value
    }
}

impl<T: IdMarker> From<Id<T>> for String {
    fn from(value: Id<T>) -> Self {
        value.value.to_string()
    }
}

impl<T: IdMarker> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.value.to_string())
    }
}

impl<'de, T: IdMarker> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IdVisitor<T: IdMarker> {
            _marker: core::marker::PhantomData<T>,
        }
        impl<'de, T: IdMarker> Visitor<'de> for IdVisitor<T> {
            type Value = Id<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a fluxer snowflake")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.into())
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let u64 = u64::try_from(v)
                    .map_err(|_| E::invalid_value(Unexpected::Signed(v), &"positive u64"))?;
                self.visit_u64(u64)
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_any(IdVisitor {
                    _marker: core::marker::PhantomData,
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u64(
                    v.parse()
                        .map_err(|_| E::invalid_value(Unexpected::Str(v), &"u64 string"))?,
                )
            }
        }

        deserializer.deserialize_any(IdVisitor {
            _marker: PhantomData,
        })
    }
}

impl<T: IdMarker> From<OffsetDateTime> for Id<T> {
    fn from(value: OffsetDateTime) -> Self {
        let millis = (value.unix_timestamp() * 1000) + i64::from(value.millisecond());
        let millis = millis - Self::FLUXER_EPOCH;
        Self {
            // We assume that the millis will not be negative and won't be too large to fit in the snowflake.
            value: millis.cast_unsigned() << 22,
            _marker: PhantomData,
        }
    }
}
