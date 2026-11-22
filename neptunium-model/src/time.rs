pub mod duration;
// pub mod timestamp;

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

pub trait TimestampExt: Sized {
    /// Returns the time string for chat messages, e.g. `<t:1778005620:R>`.
    fn time_string(&self, display_type: TimestampDisplayType) -> String;
    /// Parse a fluxer timestamp (in the same format that is returned by [`time_string`]).
    ///
    /// The string is expected to be `.trim()`ed already. It must contain exactly
    /// the timestamp.
    ///
    /// Returns `None` if the timestamp could not be parsed.
    ///
    /// [`time_string`]: Self::time_string
    #[must_use]
    fn from_md(s: &str) -> Option<Self>;
}

impl TimestampExt for jiff::Timestamp {
    fn time_string(&self, display_type: TimestampDisplayType) -> String {
        format!(
            "<t:{}:{}>",
            self.as_second(),
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

    fn from_md(s: &str) -> Option<Self> {
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
        Self::from_second(unix_timestamp).ok()
    }
}
