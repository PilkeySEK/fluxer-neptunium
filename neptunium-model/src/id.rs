use std::{fmt::Display, marker::PhantomData, num::ParseIntError};

use serde::{
    Deserialize, Serialize,
    de::{Unexpected, Visitor},
};
use time::OffsetDateTime;

use crate::id::marker::IdMarker;

mod atomic;
pub mod composite;
pub mod marker;
pub use atomic::*;

/// The Fluxer epoch. Subtract this from a UNIX timestamp (millis) to get the timestamp that should be used inside of a snowflake (`Id` in this crate).
///
/// [Source](https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/constants/src/Core.tsx#L20)
pub const FLUXER_EPOCH: u64 = 1_420_070_400_000;

/// "Snowflake" is a format for uniquely identifiable descriptors (IDs). These IDs are guaranteed to be unique across all of Fluxer, except
/// in some unique scenarios in which child objects share their parent's ID. Snowflakes are always returned as a String in the HTTP and Gateway API,
/// but are stored internally as a `u64` in this struct.
/// This struct represents a Snowflake/ID. The generic parameter is for ensuring that, for example, only an ID with `UserMarker` can be used where a
/// user ID is required.
///
/// # Examples
/// ```
/// use neptunium_model::id::{Id, marker::UserMarker};
///
/// # fn main() {
/// let some_user_id = Id::<UserMarker>::new(1130650140672000000);
/// # }
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

    /// Consumes the `Id` and returns the underlying value.
    #[must_use]
    pub fn into_inner(self) -> u64 {
        self.value
    }

    /// Every snowflake on Fluxer has the timestamp of when
    /// it was created. This method the raw timestamp information from this `Id`, in milliseconds.
    #[must_use]
    pub fn get_timestamp_raw(self) -> u64 {
        (self.value >> 22) + FLUXER_EPOCH
    }

    /// Every snowflake on Fluxer has the timestamp of when
    /// it was created. This method returns the timestamp information stored in this snowflake.
    ///
    /// # Panics
    /// Panics if creating an `OffsetDateTime` from the timestamp info in the snowflake fails.
    ///
    /// For a non-panicking version of this method, use `try_get_timestamp`.
    ///
    /// # Examples
    /// ```
    /// use neptunium_model::{time::OffsetDateTime, id::{FLUXER_EPOCH, Id, marker::UserMarker}};
    ///
    /// # fn main() {
    /// let raw_id = 1130650140672000000;
    /// let some_user_id = Id::<UserMarker>::new(raw_id);
    /// let expected_timestamp = OffsetDateTime::from_unix_timestamp_nanos(((raw_id >> 22) + FLUXER_EPOCH) as i128 * 1_000_000).unwrap();
    /// assert_eq!(some_user_id.get_timestamp(), expected_timestamp);
    /// # }
    /// ```
    #[must_use]
    pub fn get_timestamp(self) -> OffsetDateTime {
        self.try_get_timestamp().unwrap()
    }

    /// Every snowflake on Fluxer has the timestamp of when
    /// it was created. This method returns the timestamp information stored in this snowflake.
    ///
    /// This is the non-panicking version of `get_timestamp`.
    ///
    /// # Errors
    /// Returns an error if creating an `OffsetDateTime` from the timestamp info in the snowflake fails.
    pub fn try_get_timestamp(self) -> Result<OffsetDateTime, time::error::ComponentRange> {
        OffsetDateTime::from_unix_timestamp_nanos(i128::from(self.get_timestamp_raw()) * 1_000_000)
    }
    
    /// Get the worker ID of this snowflake. Due to it being 5 bits, it is
    /// represented as a `u8` here.
    #[must_use]
    pub fn get_worker_id(self) -> u8 {
        ((self.value & 0x3e_0000) >> 17) as u8
    }
    
    /// Get the process ID of this snowflake. Due to it being 5 bits, it is
    /// represented as a `u8` here.
    #[must_use]
    pub fn get_process_id(self) -> u8 {
        ((self.value & 0x1f000) >> 12) as u8
    }
    
    /// Get the "increment" of this snowflake. For every ID that is generated on a specific process,
    /// this number is incremented. Due to it being 12 bits, it is
    /// represented as a `u16` here.
    #[must_use]
    pub fn get_increment(self) -> u16 {
        (self.value & 0xfff) as u16
    }
}

impl<T: IdMarker> From<u64> for Id<T> {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl<T: IdMarker> From<Id<T>> for u64 {
    fn from(value: Id<T>) -> Self {
        value.into_inner()
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
        serializer.serialize_str(&self.to_string())
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

        // let value = serde_json::Value::deserialize(deserializer)?;
        //
        // let Ok(string) = serde_json::from_value::<String>(value.clone()) else {
        //     println!("Deserialization error while deserializing id: {:?}", value);
        //     panic!();
        // };
        //
        // Ok(Self::new(string.parse().unwrap()))

        deserializer.deserialize_any(IdVisitor {
            _marker: PhantomData,
        })
    }
}

impl<T: IdMarker> From<OffsetDateTime> for Id<T> {
    fn from(value: OffsetDateTime) -> Self {
        let millis = (value.unix_timestamp() * 1000) + i64::from(value.millisecond());
        // We assume that the millis will not be negative
        let millis = millis.cast_unsigned() - FLUXER_EPOCH;
        Self {
            // We assume that the millis will not be too large to fit in the snowflake.
            value: millis << 22,
            _marker: PhantomData,
        }
    }
}

impl<T: IdMarker> TryFrom<String> for Id<T> {
    type Error = ParseIntError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::new(value.parse()?))
    }
}

impl<T: IdMarker> TryFrom<&str> for Id<T> {
    type Error = ParseIntError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::new(value.parse()?))
    }
}
