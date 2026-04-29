use std::collections::HashMap;

use serde::Deserialize;

use crate::id::{Id, marker::GuildMarker};

#[derive(Clone, Debug)]
pub struct GuildCountsUpdateItem {
    pub member_count: usize,
    pub online_count: usize,
}

#[derive(Clone, Debug)]
pub struct GuildCountsUpdate {
    /// The gateway responds with an array but the deserialization function will change it to a `HashMap`.
    pub counts: HashMap<Id<GuildMarker>, GuildCountsUpdateItem>,
    /// If the request resulting in this response specified a nonce, this will be the same nonce.
    pub nonce: Option<String>,
}

impl<'de> Deserialize<'de> for GuildCountsUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawGuildCountsUpdateItem {
            guild_id: Id<GuildMarker>,
            member_count: usize,
            online_count: usize,
        }
        #[derive(Deserialize)]
        struct RawGuildCountsUpdate {
            counts: Vec<RawGuildCountsUpdateItem>,
            #[serde(skip_serializing_if = "Option::is_none")]
            nonce: Option<String>,
        }

        let raw = RawGuildCountsUpdate::deserialize(deserializer)?;

        let counts = raw
            .counts
            .into_iter()
            .map(|item| {
                (
                    item.guild_id,
                    GuildCountsUpdateItem {
                        member_count: item.member_count,
                        online_count: item.online_count,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(GuildCountsUpdate {
            counts,
            nonce: raw.nonce,
        })
    }
}
