use serde::{Deserialize, Serialize};

/// A representation of a duration sent to or received by the HTTP or Gateway API.
/// There is one representation implemented by this crate:
/// - `Seconds`: The duration in seconds as an integer.
pub trait DurationRepr:
    for<'de> Deserialize<'de>
    + Serialize
    + Into<std::time::Duration>
    + From<std::time::Duration>
    + From<u64>
    + Clone
    + Copy
    + PartialEq
    + Eq
{
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Seconds {
    inner: std::time::Duration,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Millis {
    inner: std::time::Duration,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MillisF64 {
    inner: std::time::Duration,
}

impl Seconds {
    /// Create a new `Seconds` duration with the specified number of seconds.
    #[must_use]
    pub fn new(seconds: u64) -> Self {
        seconds.into()
    }
}

impl Millis {
    /// Create a new `Millis` duration with the specified number of milliseconds.
    #[must_use]
    pub fn new(millis: u64) -> Self {
        millis.into()
    }
}

impl From<u64> for Seconds {
    fn from(value: u64) -> Self {
        Self {
            inner: std::time::Duration::from_secs(value),
        }
    }
}

impl From<u64> for Millis {
    fn from(value: u64) -> Self {
        Self {
            inner: std::time::Duration::from_millis(value),
        }
    }
}

impl DurationRepr for Seconds {}

impl From<Seconds> for std::time::Duration {
    fn from(value: Seconds) -> Self {
        value.inner
    }
}

impl From<std::time::Duration> for Seconds {
    fn from(value: std::time::Duration) -> Self {
        Self { inner: value }
    }
}

impl From<std::time::Duration> for Millis {
    fn from(value: std::time::Duration) -> Self {
        Self { inner: value }
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

impl DurationRepr for MillisF64 {}

impl<'de> Deserialize<'de> for MillisF64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seconds_f64 = f64::deserialize(deserializer)?;
        if seconds_f64.is_sign_negative() {
            return Err(serde::de::Error::custom("duration may not be negative"));
        }
        if seconds_f64.is_infinite() {
            return Err(serde::de::Error::custom("duration may not be infinity"));
        }
        let max_duration = std::time::Duration::MAX.as_secs_f64();
        if seconds_f64 > max_duration {
            return Err(serde::de::Error::custom(
                "duration is greater than Duration::MAX",
            ));
        }
        Ok(Self {
            inner: std::time::Duration::from_secs_f64(seconds_f64 * 1000.0),
        })
    }
}

impl Serialize for MillisF64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.inner.as_secs_f64() / 1000.0).serialize(serializer)
    }
}

impl From<std::time::Duration> for MillisF64 {
    fn from(value: std::time::Duration) -> Self {
        Self { inner: value }
    }
}

impl From<u64> for MillisF64 {
    fn from(value: u64) -> Self {
        Self {
            inner: std::time::Duration::from_millis(value),
        }
    }
}

impl From<MillisF64> for std::time::Duration {
    fn from(value: MillisF64) -> Self {
        value.inner
    }
}
