use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::time::timestamp::representations::TimestampRepr;

pub mod representations;

/// Represents a timestamp. The representation represents the behavior of this type when being serialized or deserialized.
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct Timestamp<Repr: TimestampRepr> {
    value: Repr,
}

#[expect(clippy::doc_paragraphs_missing_punctuation)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimestampDisplayType {
    /// "10:23". `t`
    ShortTime,
    /// "10:23:55". `T`
    LongTime,
    /// "5/5/2026" or "05.05.2026" depending on user language. `d`
    ShortDate,
    /// "May 5, 2026". `D`
    LongDate,
    /// "May 5, 2026, 10:00 AM". `f`
    VerboseDateWithShortTime,
    /// "Tuesday, May 5, 2026 at 10:00 AM". `F`
    VerboseDateWithDayOfWeekAndShortTime,
    /// "4/4/2026, 10:00 AM" or "05.05.2026, 10:00", depending on user language. `s`
    ShortDateAndTime,
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
                TimestampDisplayType::ShortTime => 't',
                TimestampDisplayType::LongTime => 'T',
                TimestampDisplayType::ShortDate => 'd',
                TimestampDisplayType::LongDate => 'D',
                TimestampDisplayType::VerboseDateWithShortTime => 'f',
                TimestampDisplayType::VerboseDateWithDayOfWeekAndShortTime => 'F',
                TimestampDisplayType::Relative => 'R',
                TimestampDisplayType::ShortDateAndTime => 's',
            }
        )
    }

    /// Parse a fluxer timestamp (in the same format that is returned by `Timestamp::time_string`).
    ///
    /// The string is expected to be `.trim()`ed already. It must contain exactly
    /// the timestamp.
    ///
    /// Returns `None` if the timestap could not be parsed.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        // <t:?:?>
        let s = s.strip_prefix("<t:")?;
        // ?:?>
        let s = s.strip_suffix('>')?;
        // ?:?
        let s =
            s.strip_suffix(|char| matches!(char, 't' | 'T' | 'd' | 'D' | 'f' | 'F' | 'R' | 's'))?;
        // ?:
        let unix_timestamp_str = s.strip_suffix(':')?;
        // ?
        let unix_timestamp = unix_timestamp_str.parse::<i64>().ok()?;
        if unix_timestamp < 0 {
            return None;
        }
        Some(Self::new(
            OffsetDateTime::from_unix_timestamp(unix_timestamp).ok()?,
        ))
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
