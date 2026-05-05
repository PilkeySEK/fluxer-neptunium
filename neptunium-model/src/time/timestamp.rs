use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::time::timestamp::representations::TimestampRepr;

pub mod representations;

/// Represents a timestamp. The representation represents the behavior of this type when being serialized or deserialized.
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct Timestamp<Repr: TimestampRepr> {
    value: Repr,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimestampDisplayType {
    /// "10:23".
    Time,
    /// "10:23:55".
    TimeWithSeconds,
    /// "5/5/2026" or "05.05.2026" depending on user language.
    Date,
    /// "May 5, 2026".
    VerboseDate,
    /// "May 5, 2026".
    VerboseDateWithShortTime,
    /// "Tuesday, May 5, 2026 at 10:00 AM".
    VerboseDateWithDayOfWeekAndShortTime,
    /// "5 minutes ago".
    Relative,
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

    /// Returns the time string for chat messages, e.g. `<t:1778005620:R>`.
    pub fn time_string(self, display_type: TimestampDisplayType) -> String {
        format!(
            "<t:{}:{}>",
            OffsetDateTime::from(self).unix_timestamp(),
            match display_type {
                TimestampDisplayType::Time => 't',
                TimestampDisplayType::TimeWithSeconds => 'T',
                TimestampDisplayType::Date => 'd',
                TimestampDisplayType::VerboseDate => 'D',
                TimestampDisplayType::VerboseDateWithShortTime => 'f',
                TimestampDisplayType::VerboseDateWithDayOfWeekAndShortTime => 'F',
                TimestampDisplayType::Relative => 'R',
            }
        )
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

#[cfg(feature = "chrono-timestamp-conversion")]
impl<Repr: TimestampRepr, Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for Timestamp<Repr> {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        use std::time::SystemTime;

        OffsetDateTime::from(SystemTime::from(value)).into()
    }
}

#[cfg(feature = "chrono-timestamp-conversion")]
impl<Repr: TimestampRepr> From<Timestamp<Repr>> for chrono::DateTime<chrono::Utc> {
    fn from(value: Timestamp<Repr>) -> Self {
        use std::time::SystemTime;

        SystemTime::from(OffsetDateTime::from(value)).into()
    }
}
