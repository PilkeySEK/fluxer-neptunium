use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::time::timestamp::representations::TimestampRepr;

pub mod representations;

/// Represents a timestamp. The representation represents the behavior of this type when being serialized or deserialized.
#[derive(Copy, Debug, Clone)]
pub struct Timestamp<Repr: TimestampRepr> {
    value: Repr,
}

impl<Repr: TimestampRepr> Timestamp<Repr> {
    pub fn new(value: impl Into<Repr>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Get the inner value.
    pub fn get(self) -> Repr {
        self.value
    }
}

impl<Repr: TimestampRepr + From<OffsetDateTime>> From<OffsetDateTime> for Timestamp<Repr> {
    fn from(value: OffsetDateTime) -> Self {
        Self {
            value: Repr::from(value),
        }
    }
}

impl<Repr: TimestampRepr + TryFrom<i64>> TryFrom<i64> for Timestamp<Repr> {
    type Error = <Repr as TryFrom<i64>>::Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self {
            value: Repr::try_from(value)?,
        })
    }
}

impl<Repr: TimestampRepr> From<Timestamp<Repr>> for OffsetDateTime {
    fn from(value: Timestamp<Repr>) -> Self {
        value.value.into()
    }
}

impl<Repr: TimestampRepr> Serialize for Timestamp<Repr> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de, Repr: TimestampRepr> Deserialize<'de> for Timestamp<Repr> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            value: Repr::deserialize(deserializer)?,
        })
    }
}
