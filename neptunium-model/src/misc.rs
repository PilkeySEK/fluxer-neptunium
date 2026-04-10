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
