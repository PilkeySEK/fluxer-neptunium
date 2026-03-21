use serde::{Deserialize, Serialize, de};
use time::OffsetDateTime;

/// A representation of a timestamp sent to or received by the HTTP or Gateway API.
/// There are two representations implemented by this crate:
/// - `UnixMillis`: The UNIX time in milliseconds as an integer.
/// - `Iso8601`: The time as a String which should be a valid ISO 8601 timestamp.
///
/// Both of these represenations are sent to or received by the HTTP and Gateway API making them necessary
/// to support in this crate.
pub trait TimestampRepr:
    for<'de> Deserialize<'de> + Serialize + Into<OffsetDateTime> + Clone + Copy
{
}

#[derive(Copy, Clone, Debug)]
pub struct Iso8601 {
    inner: OffsetDateTime,
}
impl TimestampRepr for Iso8601 {}

impl From<Iso8601> for OffsetDateTime {
    fn from(value: Iso8601) -> Self {
        value.inner
    }
}

impl From<OffsetDateTime> for Iso8601 {
    fn from(value: OffsetDateTime) -> Self {
        Self { inner: value }
    }
}

impl<'de> Deserialize<'de> for Iso8601 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let offset_date_time = time::serde::iso8601::deserialize(deserializer)?;
        Ok(Self {
            inner: offset_date_time,
        })
    }
}

impl Serialize for Iso8601 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        time::serde::iso8601::serialize(&self.inner, serializer)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UnixMillis {
    inner: OffsetDateTime,
}

impl TimestampRepr for UnixMillis {}

impl From<UnixMillis> for OffsetDateTime {
    fn from(value: UnixMillis) -> Self {
        value.inner
    }
}

impl From<OffsetDateTime> for UnixMillis {
    fn from(value: OffsetDateTime) -> Self {
        Self { inner: value }
    }
}

impl TryFrom<i64> for UnixMillis {
    type Error = time::error::ComponentRange;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: OffsetDateTime::from_unix_timestamp(value / 1000)?
                + time::Duration::milliseconds(value % 1000),
        })
    }
}

impl Serialize for UnixMillis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        time::serde::timestamp::serialize(&self.inner, serializer)
    }
}

impl<'de> Deserialize<'de> for UnixMillis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let millis = i64::deserialize(deserializer)?;
        // We need to create an OffsetDateTime from the time in seconds and then add the millisecond part seperately
        let offset_date_time =
            OffsetDateTime::from_unix_timestamp(millis / 1000).map_err(|_| {
                de::Error::invalid_value(de::Unexpected::Signed(millis), &"a valid UNIX timestamp")
            })? + time::Duration::milliseconds(millis % 1000);
        Ok(Self {
            inner: offset_date_time,
        })
    }
}
