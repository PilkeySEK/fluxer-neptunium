use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Builder)]
pub struct ShardInfo {
    pub id: u64,
    pub num_shards: u64,
}

impl ShardInfo {
    pub const ONE: Self = Self {
        id: 0,
        num_shards: 1,
    };

    #[must_use]
    pub fn new(id: u64, num_shards: u64) -> Self {
        Self { id, num_shards }
    }
}

impl Default for ShardInfo {
    fn default() -> Self {
        Self::ONE
    }
}

impl Serialize for ShardInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let tuple = (self.id, self.num_shards);
        tuple.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ShardInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tuple = <(u64, u64)>::deserialize(deserializer)?;
        Ok(Self {
            id: tuple.0,
            num_shards: tuple.1,
        })
    }
}
