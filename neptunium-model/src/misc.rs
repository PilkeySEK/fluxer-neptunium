use serde::{Deserialize, Serialize};

// TODO: Store this more efficiently (avoid using a string and instead find out what image hashes actually are so that they can be stored as integers or something)
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct ImageHash(String);

impl ImageHash {
    #[must_use]
    pub fn new(value: String) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// A hexadecimal color as a u32, as it is sent by the gateway.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(transparent)]
pub struct HexColor32(u32);

impl HexColor32 {
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

/// A type that can be either a String or a bool.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}

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
