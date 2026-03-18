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
    pub fn new(value: Repr) -> Self {
        Self { value }
    }

    /// Get the inner value.
    pub fn get(self) -> Repr {
        self.value
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
