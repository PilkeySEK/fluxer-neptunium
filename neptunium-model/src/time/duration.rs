use serde::{Deserialize, Serialize};

mod representation;
pub use representation::*;

/// Represents a duration. The representation represents the behavior of this type when being serialized or deserialized.
///
/// # Examples
/// ```
/// use neptunium_model::time::duration::{Duration, Seconds};
/// # fn main() {
/// let ten_seconds = Duration::new(Seconds::new(10));
/// let number_of_seconds = std::time::Duration::from(ten_seconds).as_secs();
/// assert_eq!(number_of_seconds, 10);
/// # }
/// ```
#[derive(Copy, Debug, Clone)]
pub struct Duration<Repr: DurationRepr> {
    value: Repr,
}

impl<Repr: DurationRepr> Duration<Repr> {
    #[must_use]
    pub fn new(value: Repr) -> Self {
        Self { value }
    }

    /// Create a new duration from an `u64`. The actual time depends on `Repr`s implementation of
    /// `From<u64>`. For `Seconds`, this is the number of seconds and for `Millis` this is the number
    /// of milliseconds.
    #[must_use]
    pub fn new_raw(value: u64) -> Self {
        Self {
            value: Repr::from(value),
        }
    }

    /// Get the inner value.
    #[must_use]
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
