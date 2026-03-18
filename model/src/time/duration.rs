use serde::{Deserialize, Serialize};

use crate::time::duration::representation::DurationRepr;

pub mod representation;

/// Represents a duration. The representation represents the behavior of this type when being serialized or deserialized.
#[derive(Copy, Debug, Clone)]
pub struct Duration<Repr: DurationRepr> {
    value: Repr,
}

impl<Repr: DurationRepr> Duration<Repr> {
    pub fn new(value: Repr) -> Self {
        Self { value }
    }

    /// Get the inner value.
    pub fn get(self) -> Repr {
        self.value
    }
}

impl<Repr: DurationRepr> From<Duration<Repr>> for std::time::Duration {
    fn from(value: Duration<Repr>) -> Self {
        value.value.into()
    }
}

impl<Repr: DurationRepr> Serialize for Duration<Repr> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de, Repr: DurationRepr> Deserialize<'de> for Duration<Repr> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            value: Repr::deserialize(deserializer)?,
        })
    }
}
