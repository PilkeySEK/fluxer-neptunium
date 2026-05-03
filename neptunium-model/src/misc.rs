use serde::{Deserialize, Serialize};

/// A hexadecimal color represented as a `u32`. Colors in the API are usually represented by numbers.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct HexColor(u32);

impl HexColor {
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn into_inner(self) -> u32 {
        self.0
    }

    /// Get the red part of this color, `0-255`.
    #[expect(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn red(&self) -> u8 {
        (self.into_inner() >> 16) as u8
    }

    /// Get the green part of this color, `0-255`.
    #[expect(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn green(&self) -> u8 {
        (self.into_inner() >> 8) as u8
    }

    /// Get the blue part of this color, `0-255`.
    #[expect(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn blue(&self) -> u8 {
        self.into_inner() as u8
    }
}

/// A type that can be either a String or a bool.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}

/*
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ListOrSingleValue<T> {
    One(T),
    Multiple(Vec<T>),
}

impl<T> From<T> for ListOrSingleValue<T> {
    fn from(value: T) -> Self {
        Self::One(value)
    }
}

impl<T> From<Vec<T>> for ListOrSingleValue<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Multiple(value)
    }
}
*/

macro_rules! serde_bitflags {
    ($name:ty, String($ty:ty)) => {
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Ok(Self::from_bits_truncate(
                    s.parse::<$ty>().map_err(serde::de::Error::custom)?,
                ))
            }
        }
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.bits().to_string())
            }
        }
    };
    ($name:ty, $ty:ty) => {
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self::from_bits_truncate(<$ty>::deserialize(deserializer)?))
            }
        }
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                <$ty as serde::Serialize>::serialize(&self.bits(), serializer)
            }
        }
    };
}

pub(crate) use serde_bitflags;
