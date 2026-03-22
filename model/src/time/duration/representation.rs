use serde::{Deserialize, Serialize};

/// A representation of a duration sent to or received by the HTTP or Gateway API.
/// There is one representation implemented by this crate:
/// - `Seconds`: The duration in seconds as an integer.
pub trait DurationRepr:
    for<'de> Deserialize<'de> + Serialize + Into<std::time::Duration> + Clone + Copy
{
}

#[derive(Copy, Clone, Debug)]
pub struct Seconds {
    inner: std::time::Duration,
}

#[derive(Copy, Clone, Debug)]
pub struct Millis {
    inner: std::time::Duration,
}

impl DurationRepr for Seconds {}

impl From<Seconds> for std::time::Duration {
    fn from(value: Seconds) -> Self {
        value.inner
    }
}

impl<'de> Deserialize<'de> for Seconds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            inner: std::time::Duration::from_secs(u64::deserialize(deserializer)?),
        })
    }
}

impl Serialize for Seconds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.inner.as_secs())
    }
}

impl DurationRepr for Millis {}

impl<'de> Deserialize<'de> for Millis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            inner: std::time::Duration::from_millis(u64::deserialize(deserializer)?),
        })
    }
}

impl Serialize for Millis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[expect(clippy::cast_possible_truncation)]
        serializer.serialize_u64(self.inner.as_millis() as u64)
    }
}

impl From<Millis> for std::time::Duration {
    fn from(value: Millis) -> Self {
        value.inner
    }
}
