use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone)]
pub struct InvalidSessionEvent {
    pub resumable: bool,
}

impl<'de> Deserialize<'de> for InvalidSessionEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let resumable = bool::deserialize(deserializer)?;
        Ok(Self { resumable })
    }
}

impl Serialize for InvalidSessionEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.resumable.serialize(serializer)
    }
}
